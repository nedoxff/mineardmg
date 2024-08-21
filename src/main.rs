use anyhow::Result;
use cli::{
    advanced_simple_spinner, get_storage_mode, get_thread_count, get_version, simple_spinner,
};
use cliclack::{intro, log, outro};
use client::{get_asset_index_url, get_assets};
use models::Asset;
use pack::determine_resource_pack_version;
use std::collections::HashMap;

mod cli;
mod client;
mod models;
mod pack;
mod processor;

fn main() -> Result<()> {
    intro("mineardmg")?;

    let mode = get_storage_mode()?;
    let (version, url) = get_version(mode)?;
    let threads = get_thread_count()?;
    let pack_version = advanced_simple_spinner::<i32>(
        "determining the resource pack version",
        |v| format!("found resource pack version {}", v),
        || determine_resource_pack_version(&url),
    )?;

    let assets = simple_spinner::<Result<HashMap<String, Asset>>>(
        "receiving the asset list",
        "received the asset list",
        || get_assets(&get_asset_index_url(&url)?),
    )?;
    let sounds = assets
        .into_iter()
        .filter_map(|p| (p.0.ends_with(".ogg")).then_some((p.0, p.1.hash)))
        .collect::<Vec<(String, String)>>();
    log::info(format!("found {} sounds", sounds.len()))?;

    // for chunk in sounds.chunks(threads) {}

    outro("you're done!")?;
    Ok(())
}
