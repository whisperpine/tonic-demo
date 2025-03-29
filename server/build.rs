fn main() {
    tonic_build::configure()
        .build_client(false)
        .compile_protos(&["../proto/nice.proto"], &["../proto"])
        .unwrap();
}
