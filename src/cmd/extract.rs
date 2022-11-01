use crate::archive::Archive;
use crate::partition::verify_part;
use crate::payload::Payload;
use crate::update_metadata::PartitionUpdate;
use crate::ExtractCmd;
use anyhow::{bail, Result};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::fs::OpenOptions;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::{Arc, RwLock};
use tracing::info;
use zip::read::ZipFile;

pub async fn extract(
    ExtractCmd {
        ota_path,
        out_path,
        no_verify,
    }: ExtractCmd,
) -> Result<()> {
    let mut archive = Archive::new(&ota_path)?;
    let mut payload = archive.payload()?;

    if payload.is_diff() {
        bail!("OTA image is a diff, it must be applied on top of an extracted full image");
    }

    let out_path = out_path.unwrap_or_else(|| std::env::current_dir().unwrap());
    std::fs::create_dir_all(&out_path)?;

    info!(
        "Payload contains {} partitions",
        payload.manifest.partitions.len()
    );
    let manifest = payload.manifest.clone();
    for partition in manifest.partitions {
        extract_partition(&mut payload, &out_path, partition, !no_verify).await?;
    }
    Ok(())
}

pub async fn extract_partition(
    payload: &mut Payload<ZipFile<'_>>,
    out_path: &Path,
    part: PartitionUpdate,
    verify: bool,
) -> Result<()> {
    let name = part.partition_name.clone().unwrap() + ".img";
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

    let mut futs = FuturesUnordered::new();
    let fut_limit = num_cpus::get();
    for op in part.operations {
        let handle = payload.spawn_apply_install_op(op, out.clone()).await?;
        futs.push(handle);
        if futs.len() >= fut_limit {
            futs.next().await.unwrap()??;
        }
    }
    while !futs.is_empty() {
        futs.next().await.unwrap()??;
    }

    if let Some(info) = part.new_partition_info.0 {
        if verify {
            let mut out_guard = out.write().unwrap();
            verify_part(out_guard.deref_mut(), &name, &info)?;
        }
    }
    Ok(())
}
