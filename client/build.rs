type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    tonic_build::configure()
        .build_server(false)
        .compile(&["../proto/nice.proto"], &["../proto"])?;

    Ok(())
}
