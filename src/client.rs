use anyhow::{bail, Context, Result};
use bytes::Bytes;
use reqwest::Client;
use std::collections::HashMap;

use crate::models::{Asset, AssetResponse, ClientManifest, VersionListResponse};

static VERSION_LIST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
static ASSET_CDN_URL: &str = "https://resources.download.minecraft.net";

pub fn get_versions() -> Result<VersionListResponse> {
    reqwest::blocking::get(VERSION_LIST_URL)
        .map_err(|err| anyhow::Error::from(err))
        .context("failed to get the available client versions")?
        .json::<VersionListResponse>()
        .map_err(|err| err.into())
}

pub fn get_asset_index_url(url: &str) -> Result<String> {
    reqwest::blocking::get(url)
        .map_err(|err| anyhow::Error::from(err))
        .context("failed to get the asset index url")?
        .json::<ClientManifest>()
        .map(|v| v.asset_index.url)
        .map_err(|err| err.into())
}

pub fn get_client_archive(url: &str) -> Result<Bytes> {
    let response = reqwest::blocking::get(url)?;
    let assets = response.json::<ClientManifest>()?.downloads;
    if !assets.contains_key("client") {
        bail!("the client manifest has no client jar listed");
    }

    let url = &assets.get("client").unwrap().url;
    reqwest::blocking::get(url)
        .map_err(|err| anyhow::Error::from(err))
        .context("failed to fetch the client jar archive")?
        .bytes()
        .map_err(|err| err.into())
}

pub fn get_assets(asset_index_url: &String) -> Result<HashMap<String, Asset>> {
    Ok(reqwest::blocking::get(asset_index_url)
        .map_err(|err| anyhow::Error::from(err))
        .context("failed to fetch client assets")?
        .json::<AssetResponse>()?
        .objects)
}

pub async fn get_asset_bytes(client: &Client, hash: &String) -> Result<Bytes> {
    client
        .get(format!("{}/{}/{}", ASSET_CDN_URL, &hash[..1], hash))
        .send()
        .await
        .map_err(|err| anyhow::Error::from(err))
        .context("failed to get the bytes of an asset")?
        .bytes()
        .await
        .map_err(|err| err.into())
}
