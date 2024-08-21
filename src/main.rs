use std::{collections::HashMap, error::Error, fs};

use bytes::Bytes;
use cli::{get_storage_mode, get_thread_count, get_version, simple_spinner};
use cliclack::{intro, log, outro};
use client::{get_asset_index_url, get_assets};
use models::Asset;
use processor::process_audio;

mod cli;
mod client;
mod models;
mod processor;

fn main() -> Result<(), Box<dyn Error>> {
    intro("mineardmg")?;

    let mode = get_storage_mode();
    let (version, versions) = get_version(mode);
    let threads = get_thread_count();
    let assets = simple_spinner::<HashMap<String, Asset>>(
        "receiving the asset list",
        "received the asset list",
        || get_assets(&get_asset_index_url(&version, &versions)),
    );

    let sounds = assets
        .into_iter()
        .filter_map(|p| (p.0.ends_with(".ogg")).then_some((p.0, p.1.hash)))
        .collect::<Vec<(String, String)>>();
    log::info(format!("found {} sounds", sounds.len()))?;

    for chunk in sounds.chunks(threads) {}

    outro("you're done!")?;
    Ok(())
}
