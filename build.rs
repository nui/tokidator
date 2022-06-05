fn main() {
    codegen();
}

#[cfg(test)]
fn codegen() {
    use protobuf_codegen::Codegen;

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

#[cfg(not(test))]
fn codegen() {}
