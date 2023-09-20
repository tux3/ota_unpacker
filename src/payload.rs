use crate::fec::write_verity_fec;
use crate::hashtree::write_verity_hashtree;
use crate::operation::apply_op;
use crate::partition::verify_part;
use crate::update_metadata::{DeltaArchiveManifest, InstallOperation, PartitionUpdate, Signatures};
use anyhow::{bail, Result};
use byteorder::{ReadBytesExt, BE};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use protobuf::Message;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::ops::DerefMut;
use std::path::Path;
use std::sync::{Arc, RwLock};
use tokio::task::{spawn_blocking, JoinHandle};
use tracing::{debug, info};

pub struct Payload<R: Read> {
    blob_pos: u64,
    read: R,
    pub manifest: DeltaArchiveManifest,
    pub meta_signatures: Signatures,
}

impl<R: Read> Payload<R> {
    pub fn new(mut read: R) -> Result<Payload<R>> {
        let magic = read.read_u32::<BE>()?;
        if magic.to_be_bytes() != "CrAU".as_bytes() {
            bail!("Invalid payload magic {:#x}", magic)
        }
        let format_ver = read.read_u64::<BE>()?;
        if format_ver != 2 {
            bail!("Unsupported format version {}", format_ver)
        }

        let manifest_len = read.read_u64::<BE>()?;
        let meta_sig_len = read.read_u32::<BE>()?;
        debug!(manifest_len, meta_sig_len, "Reading OTA payload");

        let manifest = {
            let mut manifest_buf = vec![0; manifest_len as usize];
            manifest_buf.resize(manifest_len as usize, 0);
            read.read_exact(&mut manifest_buf)?;
            DeltaArchiveManifest::parse_from_bytes(&manifest_buf)?
        };
        if let Some(max_timestamp) = manifest.max_timestamp {
            debug!("Payload anti-rollback max_timestamp: {}", max_timestamp)
        }

        let meta_signatures = {
            let mut signatures_buf = vec![0; meta_sig_len as usize];
            read.read_exact(&mut signatures_buf)?;
            Signatures::parse_from_bytes(&signatures_buf)?
        };

        Ok(Self {
            blob_pos: 0,
            read,
            manifest,
            meta_signatures,
        })
    }

    pub fn is_diff(&self) -> bool {
        self.manifest.minor_version.unwrap_or(0) != 0
    }

    pub async fn extract_partition(
        &mut self,
        mut src: Option<File>,
        out_path: &Path,
        part: PartitionUpdate,
        verify: bool,
    ) -> Result<()> {
        let name = part.partition_name.clone().expect("Missing partition name") + ".img";

        if let Some(src) = &mut src {
            if let Some(info) = part.old_partition_info.0 {
                if verify {
                    verify_part(src, &name, &info, true)?;
                    src.seek(SeekFrom::Start(0))?;
                }
            }
        }

        if let Some(version) = part.version.as_deref() {
            info!("Extracting {name} (version {version})",);
        } else {
            info!("Extracting {name}");
        }
        let out = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(out_path.to_owned().join(&name))?;
        let out = Arc::new(RwLock::new(out));

        let current_max_minor_version = 9;
        if self.manifest.minor_version() > current_max_minor_version {
            bail!(
                "Unsupported OTA minor version {}",
                self.manifest.minor_version()
            )
        }

        let mut futs = FuturesUnordered::new();
        let fut_limit = (num_cpus::get() as f32 * 1.5) as usize;

        let src = src.map(Arc::new);
        for op in part.operations {
            let handle = self
                .spawn_apply_install_op(op, src.clone(), out.clone())
                .await?;
            futs.push(handle);
            if futs.len() >= fut_limit {
                futs.next().await.unwrap()??;
            }
        }
        while !futs.is_empty() {
            futs.next().await.unwrap()??;
        }

        if let Some(hashtree_extent) = &part.hash_tree_extent.0 {
            let Some(data_extent) = &part.hash_tree_data_extent.0 else {
                bail!("Partition has a verity hash tree (output) extent, but no hash tree data (input) extent")
            };
            let Some(salt) = &part.hash_tree_salt else {
                bail!("Partition has a verity hash tree, but is missing a salt for the hash")
            };

            let mut out_guard = out.write().unwrap();
            write_verity_hashtree(
                out_guard.deref_mut(),
                &self.manifest,
                &part.hash_tree_algorithm.as_deref().unwrap_or("sha256"),
                salt.as_ref(),
                data_extent,
                hashtree_extent,
            )?;
        }
        if let Some(fec_extent) = &part.fec_extent.0 {
            let Some(data_extent) = &part.fec_data_extent.0 else {
                bail!("Partition has a verity hash tree (output) extent, but no hash tree data (input) extent")
            };
            let fec_roots = part.fec_roots.unwrap_or(2);

            let mut out_guard = out.write().unwrap();
            write_verity_fec(
                out_guard.deref_mut(),
                &self.manifest,
                fec_roots,
                data_extent,
                fec_extent,
            )?;
        }

        if let Some(info) = part.new_partition_info.0 {
            if verify {
                let mut out_guard = out.write().unwrap();
                verify_part(out_guard.deref_mut(), &name, &info, false)?;
            }
        }
        Ok(())
    }

    pub async fn spawn_apply_install_op(
        &mut self,
        op: InstallOperation,
        src: Option<Arc<File>>,
        out: Arc<RwLock<File>>,
    ) -> Result<JoinHandle<Result<()>>> {
        let off = op.data_offset();
        let len = op.data_length();
        if off != self.blob_pos && len != 0 {
            bail!(
                "Install operation at position {off}, but our cursor is still at {}",
                self.blob_pos
            )
        }
        let mut data = vec![0; len as usize];
        self.read.read_exact(&mut data)?;
        self.blob_pos += len;

        let block_size = self.manifest.block_size() as usize;
        let handle = spawn_blocking(move || apply_op(block_size, op, data, src, out));
        Ok(handle)
    }
}
