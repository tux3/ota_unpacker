use crate::update_metadata::PartitionInfo;
use anyhow::{bail, Result};
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use tracing::{error, info};

pub fn verify_part(extracted: &mut File, name: &str, info: &PartitionInfo) -> Result<()> {
    if let Some(size) = info.size {
        let extracted_size = extracted.metadata()?.len();
        if extracted_size != size {
            bail!("Invalid size after extracting {name}, expected {size} bytes but found {extracted_size}")
        }
    }
    if let Some(hash) = &info.hash {
        info!("Verifying hash for {name}...");
        extracted.seek(SeekFrom::Start(0))?;
        let block = &mut vec![0; 512 * 1024];
        let mut hasher = Sha256::new();
        loop {
            let rd_len = extracted.read(block)?;
            hasher.update(&block[..rd_len]);
            if rd_len == 0 {
                break;
            }
        }
        let extracted_hash = hasher.finalize_fixed();
        if hash == &extracted_hash.as_slice() {
        } else {
            error!(
                "INVALID hash for {name}: expected {:x?} but we found {:x?}",
                hex::encode(hash),
                hex::encode(extracted_hash)
            );
        }
    }
    Ok(())
}
