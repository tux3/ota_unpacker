use crate::operation::apply_op;
use crate::update_metadata::{DeltaArchiveManifest, InstallOperation, Signatures};
use anyhow::{bail, Result};
use byteorder::{ReadBytesExt, BE};
use protobuf::Message;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};
use tokio::task::{spawn_blocking, JoinHandle};
use tracing::debug;

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

    pub async fn spawn_apply_install_op(
        &mut self,
        op: InstallOperation,
        out: Arc<RwLock<File>>,
    ) -> Result<JoinHandle<Result<()>>> {
        let off = op.data_offset();
        let len = op.data_length();
        if off != self.blob_pos {
            bail!(
                "Install operation at position {off}, but our cursor is still at {}",
                self.blob_pos
            )
        }
        let mut data = vec![0; len as usize];
        self.read.read_exact(&mut data)?;
        self.blob_pos += len;

        let block_size = self.manifest.block_size() as usize;
        let handle = spawn_blocking(move || apply_op(block_size, op, data, out));
        Ok(handle)
    }
}
