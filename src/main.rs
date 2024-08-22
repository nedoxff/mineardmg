use anyhow::Result;
use bytes::Bytes;
use cli::{
    advanced_simple_spinner, get_gain, get_storage_mode, get_thread_count, get_version,
    simple_spinner,
};
use cliclack::{intro, log, outro, MultiProgress};
use client::{get_asset_index_url, get_assets};
use dashmap::DashMap;
use models::Asset;
use pack::determine_resource_pack_version;
use processor::spawn_processors;
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
    let gain = get_gain()?;
    let thread_count = get_thread_count()?;
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
        .filter_map(|p| (p.0.ends_with(".ogg")).then_some(p.1.hash))
        .collect::<Vec<String>>();
    log::info(format!("found {} sounds", sounds.len()))?;

    let multi_progress = MultiProgress::new("processing sounds");
    let processed_data: DashMap<String, Bytes> = DashMap::new();
    spawn_processors(
        gain,
        &multi_progress,
        thread_count,
        &processed_data,
        &sounds,
    );

    outro("you're done!")?;
    Ok(())
}
