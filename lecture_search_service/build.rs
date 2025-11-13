fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(
        &[
            "../proto/lecture_store.proto",
            "../proto/lecture_service.proto",
            "../proto/embedding_service.proto",
            "../proto/whisper_service.proto",
            "../proto/summary_service.proto",
            "../proto/completion_service.proto",
        ],
        &["../proto"],
    )?;
    Ok(())
}
