type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    tonic_build::configure()
        .build_client(false)
        .compile(&["../proto/nice.proto"], &["../proto"])?;

    Ok(())
}
