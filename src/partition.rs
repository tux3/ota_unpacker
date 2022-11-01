use crate::update_metadata::PartitionInfo;
use anyhow::{bail, Result};
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256};
use std::cmp::min;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use tracing::{error, info};

pub fn verify_part(file: &mut File, name: &str, info: &PartitionInfo, is_prev: bool) -> Result<()> {
    let file_size = file.metadata()?.len();
    if let Some(info_size) = info.size {
        if is_prev {
            if file_size < info_size {
                bail!("Invalid size for previous {name}, expected at least {info_size} bytes but found {file_size}")
            }
        } else if file_size != info_size {
            bail!("Invalid size after extracting {name}, expected {info_size} bytes but found {file_size}")
        }
    }
    if let Some(hash) = &info.hash {
        let mut to_read = info.size.unwrap_or(file_size) as usize;
        if is_prev {
            info!("Verifying hash for previous {name}...");
        } else {
            info!("Verifying hash for extracted {name}...");
        }
        file.seek(SeekFrom::Start(0))?;
        let mut block = vec![0; 128 * 1024];
        let mut hasher = Sha256::new();
        loop {
            let block_to_read = min(block.len(), to_read);
            let rd_len = file.read(&mut block[..block_to_read])?;
            hasher.update(&block[..rd_len]);
            to_read = to_read.saturating_sub(rd_len);
            if rd_len == 0 || to_read == 0 {
                break;
            }
        }
        let extracted_hash = hasher.finalize_fixed();
        if hash != extracted_hash.as_slice() {
            if is_prev {
                bail!(
                    "INVALID hash for previous {name}: expected {:x?} but found {:x?}",
                    hex::encode(hash),
                    hex::encode(extracted_hash)
                );
            } else {
                error!(
                    "INVALID hash for extracted {name}: expected {:x?} but found {:x?}",
                    hex::encode(hash),
                    hex::encode(extracted_hash)
                );
            }
        }
    }
    Ok(())
}
