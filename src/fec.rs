use crate::update_metadata::{DeltaArchiveManifest, Extent};
use anyhow::Result;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::os::fd::AsRawFd;
use tracing::{debug, info};

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/fec_encode_simple.rs"));
}

// fec: Forward Error Correction
pub fn write_verity_fec(
    file: &mut File,
    manifest: &DeltaArchiveManifest,
    fec_roots: u32,
    data_extent: &Extent,
    fec_extent: &Extent,
) -> Result<()> {
    let data_start = data_extent.start_block() * manifest.block_size() as u64;
    let data_end = data_start + data_extent.num_blocks() * manifest.block_size() as u64;

    let out_start = fec_extent.start_block() as usize * manifest.block_size() as usize;
    let out_end = out_start + fec_extent.num_blocks() as usize * manifest.block_size() as usize;
    info!("Computing Verity FEC (Forward Error Correction, Reed-Solomon)");
    debug!(
        "FEC input data extent: {:#x} to {:#x}, FEC output extent {:#x} to {:#x}",
        data_start, data_end, out_start, out_end
    );
    assert_eq!(data_start, 0);

    // SAFETY: The fd is valid, and we sort of have to trust the cpp side!
    file.seek(SeekFrom::Start(0))?;
    let fec_image_ctx =
        unsafe { sys::encode_simple(file.as_raw_fd(), fec_roots, data_start, data_end, 0) };
    // SAFETY: This lives as long as fec_image_ctx
    let fec_data =
        unsafe { std::slice::from_raw_parts(fec_image_ctx.fec, fec_image_ctx.fec_size as usize) };
    debug!(
        "Writing verity FEC to file (size {})",
        fec_image_ctx.fec_size
    );
    assert_eq!(fec_image_ctx.fec_size as usize, out_end - out_start);
    file.seek(SeekFrom::Start(out_start as u64))?;
    file.write_all(fec_data)?;

    // SAFETY: Ok because we didn't mangle any of the pointers in the ctx
    unsafe { sys::encode_simple_free_ctx(fec_image_ctx) };

    Ok(())
}
