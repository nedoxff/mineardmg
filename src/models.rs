use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub(crate) struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub variant: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

#[derive(Deserialize, Clone)]
pub(crate) struct LatestVersionIds {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize, Clone)]
pub struct VersionListResponse {
    pub latest: LatestVersionIds,
    pub versions: Vec<Version>,
}

#[derive(Deserialize)]
pub struct ClientManifest {
    #[serde(rename = "assetIndex")]
    pub asset_index: ClientManifestAssetIndex,
}

#[derive(Deserialize)]
pub struct ClientManifestAssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    #[serde(rename = "totalSize")]
    pub total_size: i64,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Asset {
    pub hash: String,
    pub size: i32,
}

#[derive(Deserialize)]
pub struct AssetResponse {
    pub objects: HashMap<String, Asset>,
}
