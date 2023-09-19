use std::env;
use std::path::PathBuf;

pub fn main() {
    protobuf_codegen::Codegen::new()
        .pure()
        .include("src/proto")
        .input("src/proto/update_metadata.proto")
        .cargo_out_dir("proto")
        .run_from_script();

    make_bspatch();
    make_puffin();
    make_libavb();
    make_fec();
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
        ])
        .warnings(false)
        .flag("-Wno-attributes")
        .flag("-Wno-deprecated-declarations")
        .include("/usr/include/android/")
        .include("fec")
        .include("fec/libfec/include")
        .include("fec/ext4_utils/include")
        .include("fec/squashfs_utils")
        .include("fec/squashfs-tools")
        .compile("fec");
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
