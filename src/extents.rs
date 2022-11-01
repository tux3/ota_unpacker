use crate::update_metadata::Extent;
use anyhow::{bail, Result};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};

const SENTINEL_SPARSE_BLOCKS: u64 = u64::MAX;

pub fn zero_extents<'a>(
    dst_extents: impl IntoIterator<Item = &'a Extent>,
    mut dst: Arc<RwLock<File>>,
    block_size: usize,
) -> Result<()> {
    for extent in dst_extents.into_iter() {
        let zeroes = vec![0u8; extent.num_blocks() as usize * block_size];
        write_extent(&mut zeroes.as_slice(), extent, &mut dst, block_size)?;
    }
    Ok(())
}

pub fn write_extents<'a>(
    mut src: impl Read,
    dst_extents: impl IntoIterator<Item = &'a Extent>,
    mut dst: Arc<RwLock<File>>,
    block_size: usize,
) -> Result<()> {
    for extent in dst_extents.into_iter() {
        write_extent(&mut src, extent, &mut dst, block_size)?;
    }
    Ok(())
}

#[cfg(unix)]
pub fn write_extent(
    src: &mut impl Read,
    dst_extent: &Extent,
    dst: &mut Arc<RwLock<File>>,
    block_size: usize,
) -> Result<()> {
    use std::os::unix::fs::FileExt;

    let start = dst_extent.start_block();
    if start == SENTINEL_SPARSE_BLOCKS {
        bail!("Writing sparse blocks not implemented");
    }

    let dst_guard = dst.read().unwrap();
    let mut pos = start * block_size as u64;

    // TODO: Read bigger unit than block_size, e.g. 128k at a time
    let mut block = vec![0; block_size];
    for _ in 0..dst_extent.num_blocks() {
        read_block(src, &mut block)?;
        dst_guard.write_all_at(&block, pos)?;
        pos += block_size as u64;
    }
    Ok(())
}

#[cfg(not(unix))]
pub fn write_extent(
    mut src: &mut impl Read,
    dst_extent: &Extent,
    dst: &mut Arc<RwLock<File>>,
    block_size: usize,
) -> Result<()> {
    use std::io::{Read, Seek, SeekFrom, Write};

    let start = dst_extent.start_block();
    if start == SENTINEL_SPARSE_BLOCKS {
        bail!("Writing sparse blocks not implemented");
    }

    let mut dst_guard = dst.write().unwrap();
    dst_guard.seek(SeekFrom::Start(start * block_size as u64))?;

    let mut block = vec![0; block_size];
    for _ in 0..dst_extent.num_blocks() {
        read_block(&mut src, &mut block)?;
        dst_guard.write_all(&block)?;
    }
    Ok(())
}

pub fn copy_extents(
    src_extents: &[Extent],
    src: Arc<File>,
    dst_extents: &[Extent],
    mut dst: Arc<RwLock<File>>,
    block_size: usize,
) -> Result<()> {
    let src_blocks = src_extents
        .iter()
        .fold(0, |sum, ext| sum + ext.num_blocks());
    let dst_blocks = dst_extents
        .iter()
        .fold(0, |sum, ext| sum + ext.num_blocks());
    if src_blocks != dst_blocks {
        bail!("Cannot copy {src_blocks} source blocks into {dst_blocks} destination blocks");
    }
    let mut data = Vec::new();
    for ext in src_extents {
        read_extent_all(ext, &src, &mut data, block_size)?;
    }

    let mut to_write = data.as_slice();
    for extent in dst_extents {
        write_extent(&mut to_write, extent, &mut dst, block_size)?;
    }
    Ok(())
}

#[cfg(unix)]
pub fn read_extent_all(
    src_extent: &Extent,
    src: &Arc<File>,
    dst: &mut Vec<u8>,
    block_size: usize,
) -> Result<()> {
    use std::os::unix::fs::FileExt;

    let src_start = src_extent.start_block();
    if src_start == SENTINEL_SPARSE_BLOCKS {
        bail!("Reading sparse blocks not implemented");
    }
    let read_len = src_extent.num_blocks() as usize * block_size;
    let old_len = dst.len();
    dst.resize(old_len + read_len, 0);
    src.read_exact_at(&mut dst[old_len..], src_start * block_size as u64)?;
    Ok(())
}

fn read_block(src: &mut impl Read, mut block: &mut [u8]) -> Result<()> {
    loop {
        let count = src.read(block)?;
        block = &mut block[count..];
        let remaining = block.len();
        if remaining == 0 {
            break;
        }
        if count == 0 {
            bail!("Zero-padding incomplete blocks is not implemented");
        }
    }
    Ok(())
}
