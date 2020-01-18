fn main() {
    if let Err(e) = protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/protos",
        input: &["protos/token.proto"],
        includes: &["protos"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    }) {
        println!("cargo:warning=fail to generate protobuf file from source: {:?}", e);
    }
}