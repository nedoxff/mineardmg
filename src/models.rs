use serde::Deserialize;
use std::collections::HashMap;

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
    pub downloads: HashMap<String, ClientManifestAsset>,
}

#[derive(Deserialize)]
pub struct ClientManifestAsset {
    pub sha1: String,
    pub size: i32,
    pub url: String,
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

#[derive(Deserialize)]
pub struct ClientVersionInformation {
    pub id: String,
    pub name: String,
    pub world_version: i32,
    pub series_id: String,
    pub protocol_version: i32,
    pub pack_version: ClientPackVersion,
    pub build_time: String,
    pub stable: bool,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ClientPackVersion {
    Old(i32),
    New { resource: i32, data: i32 },
}
