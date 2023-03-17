use std::{fs::metadata, path::PathBuf};

#[test]
fn draft_test() {
    let data_dir = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "data"]);
    println!("{}", data_dir.to_str().unwrap());
    if let Err(_) = metadata(&data_dir) {
        panic!("file not found");
    }
}
