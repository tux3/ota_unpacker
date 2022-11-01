use crate::extents::write_extents;
use crate::update_metadata::install_operation::Type;
use crate::update_metadata::InstallOperation;
use anyhow::Result;
use bzip2::read::BzDecoder;
use std::fs::File;
use std::sync::{Arc, RwLock};
use tracing::trace;
use xz2::read::XzDecoder;

pub fn apply_op(
    block_size: usize,
    op: InstallOperation,
    data: Vec<u8>,
    out: Arc<RwLock<File>>,
) -> Result<()> {
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
        Type::MOVE => todo!(),
        Type::BSDIFF => todo!(),
        Type::SOURCE_COPY => todo!(),
        Type::SOURCE_BSDIFF => todo!(),
        Type::REPLACE_XZ => apply_replace_xz(block_size, &op, &data, out),
        Type::ZERO => todo!(),
        Type::DISCARD => todo!(),
        Type::BROTLI_BSDIFF => todo!(),
        Type::PUFFDIFF => todo!(),
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
