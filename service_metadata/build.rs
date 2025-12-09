fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path = std::env::var("PROTO_PATH").unwrap_or_else(|_| "../proto".to_string());

    tonic_prost_build::configure()
        .protoc_arg(format!("-I={}", proto_path))
        .compile_protos(&["metadata_service.proto"], &["proto"])?;
    Ok(())
}
