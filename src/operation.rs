use crate::bspatch::bspatch;
use crate::extents::{copy_extents, read_extent_all, write_extents, zero_extents};
use crate::puffpatch::puffpatch;
use crate::update_metadata::install_operation::Type;
use crate::update_metadata::InstallOperation;
use anyhow::{anyhow, bail, Result};
use bzip2::read::BzDecoder;
use std::fs::File;
use std::sync::{Arc, RwLock};
use tracing::trace;
use xz2::read::XzDecoder;

fn assert_src(src: Option<Arc<File>>) -> Result<Arc<File>> {
    src.ok_or_else(|| {
        anyhow!(
            "This is an incremental update OTA. You must use apply it \
            on top of the previous OTA, it cannot be extracted by itself."
        )
    })
}

pub fn apply_op(
    block_size: usize,
    op: InstallOperation,
    data: Vec<u8>,
    src: Option<Arc<File>>,
    out: Arc<RwLock<File>>,
) -> Result<()> {
    if !op.has_type() {
        bail!("Diff operation without a valid type (or our protobuf definition is too old)");
    }
    let op_type = op.type_();
    trace!(
        "Install op {:?}, pos {}, len {}",
        op_type,
        op.data_offset(),
        op.data_length()
    );

    match op_type {
        Type::REPLACE => apply_replace(block_size, &op, &data, out),
        Type::REPLACE_BZ => apply_replace_bz(block_size, &op, &data, out),
        Type::MOVE => bail!("Deprecated MOVE op not implemented"),
        Type::BSDIFF => bail!("Deprecated BSDIFF op not implemented"),
        Type::SOURCE_COPY => apply_src_copy(block_size, &op, assert_src(src)?, out),
        Type::SOURCE_BSDIFF => apply_src_bsdiff(block_size, &op, &data, assert_src(src)?, out),
        Type::REPLACE_XZ => apply_replace_xz(block_size, &op, &data, out),
        Type::ZERO => apply_zeroes(block_size, &op, out),
        Type::DISCARD => bail!("DISCARD op not implemented"),
        Type::BROTLI_BSDIFF => apply_src_bsdiff(block_size, &op, &data, assert_src(src)?, out),
        Type::PUFFDIFF => apply_puffdiff(block_size, &op, &data, assert_src(src)?, out),
        Type::ZUCCHINI => bail!("ZUCCHINI op not implemented"),
        Type::LZ4DIFF_BSDIFF => bail!("LZ4DIFF_BSDIFF op not implemented"),
        Type::LZ4DIFF_PUFFDIFF => bail!("LZ4DIFF_PUFFDIFF op not implemented"),
    }
}

fn apply_replace(
    block_size: usize,
    op: &InstallOperation,
    data: &[u8],
    out: Arc<RwLock<File>>,
) -> Result<()> {
    write_extents(data, &op.dst_extents, out, block_size)
}

fn apply_replace_bz(
    block_size: usize,
    op: &InstallOperation,
    data: &[u8],
    out: Arc<RwLock<File>>,
) -> Result<()> {
    let decompressor = BzDecoder::new(data);
    write_extents(decompressor, &op.dst_extents, out, block_size)?;
    Ok(())
}

fn apply_src_copy(
    block_size: usize,
    op: &InstallOperation,
    src: Arc<File>,
    out: Arc<RwLock<File>>,
) -> Result<()> {
    copy_extents(&op.src_extents, src, &op.dst_extents, out, block_size)
}

fn apply_replace_xz(
    block_size: usize,
    op: &InstallOperation,
    data: &[u8],
    out: Arc<RwLock<File>>,
) -> Result<()> {
    let decompressor = XzDecoder::new(data);
    write_extents(decompressor, &op.dst_extents, out, block_size)?;
    Ok(())
}

fn apply_zeroes(block_size: usize, op: &InstallOperation, out: Arc<RwLock<File>>) -> Result<()> {
    zero_extents(&op.dst_extents, out, block_size)?;
    Ok(())
}

fn apply_src_bsdiff(
    block_size: usize,
    op: &InstallOperation,
    data: &[u8],
    src: Arc<File>,
    out: Arc<RwLock<File>>,
) -> Result<()> {
    let mut src_buf = Vec::new();
    for ext in &op.src_extents {
        read_extent_all(ext, &src, &mut src_buf, block_size)?;
    }
    let dst_data = bspatch(&src_buf, data)?;
    write_extents(dst_data.as_slice(), &op.dst_extents, out, block_size)
}

fn apply_puffdiff(
    block_size: usize,
    op: &InstallOperation,
    data: &[u8],
    src: Arc<File>,
    out: Arc<RwLock<File>>,
) -> Result<()> {
    let mut src_buf = Vec::new();
    for ext in &op.src_extents {
        read_extent_all(ext, &src, &mut src_buf, block_size)?;
    }
    let dst_data = puffpatch(&src_buf, data)?;
    write_extents(dst_data.as_slice(), &op.dst_extents, out, block_size)
}
