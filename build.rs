use std::env;
use std::path::PathBuf;

pub fn main() {
    protobuf_codegen::Codegen::new()
        .pure()
        .include("src/proto")
        .input("src/proto/update_metadata.proto")
        .cargo_out_dir("proto")
        .run_from_script();

    make_libbase();
    make_sparse();
    make_bspatch();
    make_puffin();
    make_libavb();
    make_fec();
}

fn make_libbase() {
    cc::Build::new()
        .cpp(true)
        .files([
            "fec/libbase/abi_compatibility.cpp",
            "fec/libbase/chrono_utils.cpp",
            "fec/libbase/cmsg.cpp",
            "fec/libbase/file.cpp",
            "fec/libbase/hex.cpp",
            "fec/libbase/logging.cpp",
            "fec/libbase/mapped_file.cpp",
            "fec/libbase/parsebool.cpp",
            "fec/libbase/parsenetaddress.cpp",
            "fec/libbase/posix_strerror_r.cpp",
            "fec/libbase/process.cpp",
            "fec/libbase/properties.cpp",
            "fec/libbase/stringprintf.cpp",
            "fec/libbase/strings.cpp",
            "fec/libbase/threads.cpp",
            "fec/libbase/test_utils.cpp",
        ])
        .warnings(false)
        .flag("-Wno-attributes")
        .flag("-Wno-ignored-attributes")
        .define("__builtin_available(a,b)", "0")
        .include("fec/libbase/include")
        .include("fec/liblog/include")
        .compile("android_libbase");
}

fn make_sparse() {
    cc::Build::new()
        .cpp(true)
        .files([
            "fec/libsparse/append2simg.cpp",
            "fec/libsparse/backed_block.cpp",
            "fec/libsparse/img2simg.cpp",
            "fec/libsparse/output_file.cpp",
            "fec/libsparse/simg2img.cpp",
            "fec/libsparse/sparse.cpp",
            "fec/libsparse/sparse_crc32.cpp",
            "fec/libsparse/sparse_err.cpp",
            "fec/libsparse/sparse_fuzzer.cpp",
            "fec/libsparse/sparse_read.cpp",
        ])
        .warnings(false)
        .flag("-Wno-attributes")
        .include("fec/libsparse/include")
        .include("fec/libbase/include")
        .compile("android_sparse");
}

fn make_fec() {
    cc::Build::new()
        .cpp(true)
        .files([
            "fec/libfec/avb_utils.cpp",
            "fec/libfec/fec_open.cpp",
            "fec/libfec/fec_process.cpp",
            "fec/libfec/fec_read.cpp",
            "fec/libfec/fec_verity.cpp",
            "fec/ext4_utils/ext4_sb.cpp",
            "fec/ext4_utils/ext4_utils.cpp",
            "fec/ext4_utils/wipe.cpp",
            "fec/squashfs_utils/squashfs_utils.c",
            "fec/image.cpp",
            "fec/klog_write_stub.cpp",
            "fec/encode_simple.cpp",
        ])
        .warnings(false)
        .flag("-Wno-attributes")
        .flag("-Wno-deprecated-declarations")
        .include("fec/libsparse/include")
        .include("fec/libbase/include")
        .include("fec")
        .include("fec/libfec/include")
        .include("fec/ext4_utils/include")
        .include("fec/squashfs_utils")
        .include("fec/squashfs-tools")
        .compile("android_fec");

    // This one is the upstream libfec (not ours, not android's)
    println!("cargo:rustc-link-lib=fec");

    println!("cargo:rerun-if-changed=fec/encode_simple.h");
    println!("cargo:rerun-if-changed=fec/encode_simple.cpp");
    let bindings = bindgen::Builder::default()
        .clang_args(["-x", "c++"])
        .clang_arg("-Ifec")
        .clang_arg("-DINCLUDE_IMAGE_STRUCT_ONLY")
        .allowlist_function(".*encode_simple.*")
        .header("fec/encode_simple.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate fec_encode_simple bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("fec_encode_simple.rs"))
        .expect("Couldn't write fec_encode_simple bindings!");
}

fn make_libavb() {
    cc::Build::new()
        .cpp(false)
        .files([
            "fec/libavb/avb_chain_partition_descriptor.c",
            "fec/libavb/avb_cmdline.c",
            "fec/libavb/avb_crc32.c",
            "fec/libavb/avb_crypto.c",
            "fec/libavb/avb_descriptor.c",
            "fec/libavb/avb_footer.c",
            "fec/libavb/avb_hash_descriptor.c",
            "fec/libavb/avb_hashtree_descriptor.c",
            "fec/libavb/avb_kernel_cmdline_descriptor.c",
            "fec/libavb/avb_property_descriptor.c",
            "fec/libavb/avb_rsa.c",
            "fec/libavb/avb_slot_verify.c",
            "fec/libavb/avb_sysdeps_posix.c",
            "fec/libavb/avb_util.c",
            "fec/libavb/avb_vbmeta_image.c",
            "fec/libavb/avb_version.c",
            "fec/libavb/sha/sha256_impl.c",
            "fec/libavb/sha/sha512_impl.c",
        ])
        .warnings(false)
        .define("AVB_COMPILATION", "")
        .include("fec/libavb/sha")
        .compile("avb");
}

fn make_bspatch() {
    cc::Build::new()
        .cpp(true)
        .files([
            "bsdiff/brotli_decompressor.cc",
            "bsdiff/bspatch.cc",
            "bsdiff/bz2_decompressor.cc",
            "bsdiff/buffer_file.cc",
            "bsdiff/decompressor_interface.cc",
            "bsdiff/extents.cc",
            "bsdiff/extents_file.cc",
            "bsdiff/file.cc",
            "bsdiff/logging.cc",
            "bsdiff/memory_file.cc",
            "bsdiff/patch_reader.cc",
            "bsdiff/sink_file.cc",
            "bsdiff/utils.cc",
        ])
        .warnings(false)
        .static_flag(true)
        .include(".")
        .include("bsdiff/include")
        .compile("bspatch");

    println!("cargo:rerun-if-changed=bsdiff/include/bsdiff/bspatch.h");
    println!("cargo:rustc-link-lib=brotlidec");
    let bindings = bindgen::Builder::default()
        .clang_arg("-xc++")
        .clang_arg("-Ibsdiff/include")
        .allowlist_function(".*bspatch_sink_c.*")
        .header("bsdiff/include/bsdiff/bspatch.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bspatch bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bspatch.rs"))
        .expect("Couldn't write bspatch bindings!");
}

fn make_puffin() {
    cc::Build::new()
        .cpp(true)
        .files([
            "puffin/src/bit_reader.cc",
            "puffin/src/bit_writer.cc",
            "puffin/src/extent_stream.cc",
            "puffin/src/file_stream.cc",
            "puffin/src/huffer.cc",
            "puffin/src/huffman_table.cc",
            "puffin/src/memory_stream.cc",
            "puffin/src/sink_stream.cc",
            "puffin/src/puffer.cc",
            "puffin/src/puffpatch.cc",
            "puffin/src/puff_reader.cc",
            "puffin/src/puff_writer.cc",
            "puffin/src/puffin_stream.cc",
            "puffin/src/utils.cc",
            "puffin/src/puffin.pb.cc",
        ])
        .warnings(false)
        .static_flag(true)
        .include(".")
        .include("puffin/src/include")
        .include("bsdiff/include")
        .compile("puffin");

    println!("cargo:rerun-if-changed=puffin/puffin/src/include/puffin/puffpatch.h");
    println!("cargo:rustc-link-lib=protobuf-lite");
    let bindings = bindgen::Builder::default()
        .clang_args(["-x", "c++"])
        .clang_arg("-Ipuffin/puffin/src/include")
        .allowlist_function(".*puffpatch_c.*")
        .header("puffin/puffin/src/include/puffin/puffpatch.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate puffin bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("puffpatch.rs"))
        .expect("Couldn't write puffin bindings!");
}
