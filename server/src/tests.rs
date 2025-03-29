use std::fs::metadata;
use std::path::PathBuf;

#[test]
fn draft_test() {
    let data_dir = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "data"]);
    println!("{}", data_dir.to_str().unwrap());
    if metadata(&data_dir).is_err() {
        panic!("file not found");
    }
}
