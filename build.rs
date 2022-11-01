pub fn main()
{
    protobuf_codegen::Codegen::new()
        .pure()
        .include("src/proto")
        .input("src/proto/update_metadata.proto")
        .cargo_out_dir("proto")
        .run_from_script();
}
