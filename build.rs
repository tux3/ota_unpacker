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
        .clang_args(["-x", "c++"])
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
