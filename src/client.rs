use std::collections::HashMap;

use reqwest::Error;

use crate::models::{Asset, AssetResponse, ClientManifest, Version, VersionListResponse};
static VERSION_LIST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

pub fn get_versions() -> Result<VersionListResponse, Error> {
    reqwest::blocking::get(VERSION_LIST_URL)?.json::<VersionListResponse>()
}

pub fn get_asset_index_url(version: &str, versions: &[Version]) -> String {
    let url = &versions.into_iter().find(|&v| v.id == version).unwrap().url;

    let response = reqwest::blocking::get(url);
    match response {
        Ok(manifest) => manifest.json::<ClientManifest>().unwrap().asset_index.url,
        Err(err) => panic!("failed to get the asset index url: {}", err),
    }
}

pub fn get_assets(asset_index_url: &String) -> HashMap<String, Asset> {
    let response = reqwest::blocking::get(asset_index_url);
    match response {
        Ok(assets) => assets.json::<AssetResponse>().unwrap().objects,
        Err(err) => panic!("failed to get client assets: {}", err),
    }
}
