use protobuf_codegen::Codegen;

fn main() {
    if let Err(e) = Codegen::new()
        .out_dir("src/protos")
        .inputs(&["protos/token.proto"])
        .includes(&["protos"])
        .run()
    {
        println!(
            "cargo:warning=fail to generate protobuf file from source: {:?}",
            e
        );
    }
}
