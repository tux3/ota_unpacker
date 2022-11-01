use crate::update_metadata::Extent;
use anyhow::{bail, Result};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};

const SENTINEL_SPARSE_BLOCKS: u64 = u64::MAX;

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
    mut src: &mut impl Read,
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

    let mut block = vec![0; block_size];
    for _ in 0..dst_extent.num_blocks() {
        read_block(&mut src, &mut block)?;
        dst_guard.write_at(&block, pos)?;
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

fn read_block(src: &mut impl Read, mut block: &mut [u8]) -> Result<()> {
    loop {
        let count = src.read(block)?;
        block = &mut block[count..];
        let remaining = block.len();
        if remaining == 0 {
            break;
        }
        if count == 0 {
            block.fill(0);
            break;
        }
    }
    Ok(())
}
