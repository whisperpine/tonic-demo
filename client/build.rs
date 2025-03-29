fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["../proto/nice.proto"], &["../proto"])
        .unwrap();
}
