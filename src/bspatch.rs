use anyhow::{bail, Result};
use std::ffi::c_void;

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bspatch.rs"));
}

#[derive(Copy, Clone)]
struct VecParts {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

unsafe extern "C" fn sink_thunk(parts: *const c_void, ptr: *const u8, len: u64) -> u64 {
    let parts = unsafe { &mut *(parts as *mut VecParts) };
    let mut buf = Vec::from_raw_parts(parts.ptr, parts.len, parts.cap);
    let data = unsafe { core::slice::from_raw_parts(ptr, len as usize) };
    buf.extend_from_slice(data);

    parts.ptr = buf.as_mut_ptr();
    parts.len = buf.len();
    parts.cap = buf.capacity();
    Vec::leak(buf);
    len
}

pub fn bspatch(old_data: &[u8], patch_data: &[u8]) -> Result<Vec<u8>> {
    let mut buf = Vec::<u8>::new();
    let mut parts = VecParts {
        ptr: buf.as_mut_ptr(),
        len: buf.len(),
        cap: buf.capacity(),
    };
    Vec::leak(buf);

    let ret = unsafe {
        sys::bsdiff_bspatch_sink_c(
            old_data.as_ptr(),
            old_data.len(),
            patch_data.as_ptr(),
            patch_data.len(),
            &mut parts as *mut VecParts as *const c_void,
            Some(sink_thunk),
        )
    };

    let buf = unsafe { Vec::from_raw_parts(parts.ptr, parts.len, parts.cap) };
    if ret != 0 {
        bail!("bspatch failed with err {}", ret)
    }
    Ok(buf)
}
