use crate::archive::Archive;
use crate::ExtractCmd;
use anyhow::{bail, Result};
use tracing::info;

pub async fn extract(
    ExtractCmd {
        ota,
        out: out_path,
        no_verify,
    }: ExtractCmd,
) -> Result<()> {
    let mut archive = Archive::new(&ota)?;
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
        payload
            .extract_partition(None, &out_path, partition, !no_verify)
            .await?;
    }
    Ok(())
}
