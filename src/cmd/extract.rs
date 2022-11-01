use crate::archive::Archive;
use crate::payload::Payload;
use crate::update_metadata::PartitionUpdate;
use crate::ExtractCmd;
use anyhow::{bail, Result};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, RwLock};
use tracing::info;
use zip::read::ZipFile;

pub async fn extract(ExtractCmd { ota_path, out_path }: ExtractCmd) -> Result<()> {
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
        extract_partition(&mut payload, &out_path, partition).await?;
    }
    Ok(())
}

pub async fn extract_partition(
    payload: &mut Payload<ZipFile<'_>>,
    out_path: &Path,
    part: PartitionUpdate,
) -> Result<()> {
    let name = part.partition_name.clone().unwrap() + ".img";
    if let Some(version) = part.version.as_deref() {
        info!("Extracting {name} (version {version})",);
    } else {
        info!("Extracting {name}");
    }
    let out = File::create(out_path.to_owned().join(name))?;
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
    Ok(())
}
