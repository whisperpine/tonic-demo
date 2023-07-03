use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Feature {
    location: Location,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: i32,
    longitude: i32,
}

pub fn load() -> Result<Vec<crate::proto::Feature>> {
    let path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "data/route_guide_db.json"]);
    let json_file = std::fs::File::open(path)?;
    let features: Vec<Feature> = serde_json::from_reader(json_file)?;

    let decoded = features
        .into_iter()
        .map(|feature| crate::proto::Feature {
            name: feature.name,
            location: Some(crate::proto::Point {
                longitude: feature.location.longitude,
                latitude: feature.location.latitude,
            }),
        })
        .collect();

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_test() {
        load().unwrap();
    }
}
