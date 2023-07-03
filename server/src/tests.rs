use std::{fs::metadata, path::PathBuf};

#[test]
fn draft_test() {
    let data_dir = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "data"]);
    println!("{}", data_dir.to_str().unwrap());
    if metadata(&data_dir).is_err() {
        panic!("file not found");
    }
}

#[test]
fn math_test() {
    assert_eq!(25, 5i32.pow(2));
}

#[tokio::test]
async fn async_stream_test() {
    use tokio_stream::StreamExt;

    let s = async_stream::stream! {
        for i in 0..3 {
            yield i;
        }
    };

    let mut s = std::pin::pin!(s);

    while let Some(value) = s.next().await {
        println!("got {}", value);
    }
}
