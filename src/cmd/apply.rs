use crate::archive::Archive;
use crate::ApplyCmd;
use anyhow::{Context, Result};
use std::fs::File;
use tracing::{info, warn};

pub async fn apply(
    ApplyCmd {
        ota,
        previous,
        out: out_path,
        no_verify,
    }: ApplyCmd,
) -> Result<()> {
    let mut archive = Archive::new(&ota)?;
    let mut payload = archive.payload()?;

    if !payload.is_diff() {
        warn!("This seems to be a full OTA and not a diff. Extracting anyways.");
    }

    let out_path = out_path.unwrap_or_else(|| std::env::current_dir().unwrap());
    std::fs::create_dir_all(&out_path)?;

    info!(
        "Payload contains {} partitions",
        payload.manifest.partitions.len()
    );
    let manifest = payload.manifest.clone();
    for partition in manifest.partitions {
        let name = partition
            .partition_name
            .clone()
            .expect("Missing partition name")
            + ".img";
        let src = File::open(previous.clone().join(&name))
            .with_context(|| format!("Opening previous OTA partition {name}"))?;
        payload
            .extract_partition(Some(src), &out_path, partition, !no_verify)
            .await?;
    }
    Ok(())
}
