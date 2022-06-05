use protobuf_codegen::Codegen;

fn main() {
    if let Err(err) = Codegen::new()
        .pure()
        .out_dir("src/protos")
        .inputs(&["protos/token.proto"])
        .includes(&["protos"])
        .run()
    {
        println!(
            "cargo:error=fail to generate protobuf file from source: {:?}",
            err
        );
    }
}
