use crate::update_metadata::{DeltaArchiveManifest, Extent};
use anyhow::{bail, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BorrowedBuf, Read, Seek, SeekFrom, Write};
use tracing::{debug, info};

struct Level {
    pub offset: usize,
    pub size: usize,
}

pub fn write_verity_hashtree(
    file: &mut File,
    manifest: &DeltaArchiveManifest,
    hash_algorithm_name: &str,
    salt: &[u8],
    data_extent: &Extent,
    hashtree_extent: &Extent,
) -> Result<()> {
    if hash_algorithm_name != "sha256" {
        bail!("Verity hash algorithm {hash_algorithm_name} not supported")
    }

    let data_start = data_extent.start_block() as usize * manifest.block_size() as usize;
    let data_end = data_start + data_extent.num_blocks() as usize * manifest.block_size() as usize;

    let out_start = hashtree_extent.start_block() as usize * manifest.block_size() as usize;
    let out_end =
        out_start + hashtree_extent.num_blocks() as usize * manifest.block_size() as usize;

    let image_size = data_end - data_start;
    let block_size = 4096;
    let hash_size = 32; // Hardcoded for sha256 for now
    let levels_meta = compute_levels_metadata(image_size, block_size, hash_size);
    let tree_size = levels_meta.iter().fold(0, |a, l| a + l.size);

    info!("Computing verity hash tree ({hash_algorithm_name}, size {tree_size})");
    debug_assert_eq!(tree_size, out_end - out_start);

    file.seek(SeekFrom::Start(data_start as u64))?;
    let file_data_extent = file.take((data_end - data_start) as u64);
    let mut levels_data = Vec::new();
    levels_data.push(hash_level(file_data_extent, block_size, salt)?);

    while levels_data.last().unwrap().len() > block_size {
        let prev_level = levels_data.last().unwrap();
        let new_level_data = hash_level(prev_level.as_slice(), block_size, salt)?;
        levels_data.push(new_level_data);
    }

    debug!("Writing verity hash tree to file");
    file.seek(SeekFrom::Start(out_start as u64))?;
    for level_data in levels_data.iter().rev() {
        file.write_all(level_data)?;
    }
    Ok(())
}

fn hash_level(mut source: impl Read, block_size: usize, salt: &[u8]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut buf_vec = Vec::with_capacity(block_size);
    let mut buf = BorrowedBuf::from(buf_vec.spare_capacity_mut());
    while source.read_buf_exact(buf.unfilled()).is_ok() {
        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(buf.filled());
        let hash = hasher.finalize();
        out.extend_from_slice(&hash);
        buf.clear();
    }

    if buf.len() > 0 && buf.len() != block_size {
        let pad_size = block_size - buf.len();
        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(buf.filled());
        // NOTE: The BorrowedBuf never modified buf_vec.len(), so resize is guaranteed to write zeroes
        //      (even though the data was already initialized, the Vec doesn't know that!)
        buf_vec.resize(pad_size, 0);
        hasher.update(buf_vec.as_slice());
        let hash = hasher.finalize();
        out.extend_from_slice(&hash);
    }

    out.resize(round_up_to_multiple(out.len(), block_size), 0);
    Ok(out)
}

fn compute_levels_metadata(image_size: usize, block_size: usize, hash_size: usize) -> Vec<Level> {
    let mut levels = Vec::new();
    let mut last_level_size = image_size;
    while last_level_size > block_size {
        let num_blocks = (last_level_size + block_size - 1) / block_size;
        let size = round_up_to_multiple(num_blocks * hash_size, block_size);

        levels.push(Level { offset: 0, size });
        last_level_size = size;
    }

    for n in 0..levels.len() {
        levels[n].offset = levels.iter().skip(n + 1).fold(0, |a, l| a + l.size);
    }
    levels
}

fn round_up_to_multiple(number: usize, size: usize) -> usize {
    let remainder = number % size;
    if remainder == 0 {
        number
    } else {
        number + size - remainder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_size_big() {
        let levels = compute_levels_metadata(2050252800, 4096, 32);
        assert_eq!(levels.len(), 3);
        assert_eq!(levels[0].offset, 131072);
        assert_eq!(levels[0].size, 16019456);
        assert_eq!(levels[1].offset, 4096);
        assert_eq!(levels[1].size, 126976);
        assert_eq!(levels[2].offset, 0);
        assert_eq!(levels[2].size, 4096);
    }

    #[test]
    fn level_size_small() {
        let levels = compute_levels_metadata(1327104, 4096, 32);
        assert_eq!(levels.len(), 2);
        assert_eq!(levels[0].offset, 4096);
        assert_eq!(levels[0].size, 12288);
        assert_eq!(levels[1].offset, 0);
        assert_eq!(levels[1].size, 4096);
    }
}
