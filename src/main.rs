use std::collections::HashMap;

use cli::{get_storage_mode, get_thread_count, get_version, simple_spinner};
use client::{get_asset_index_url, get_assets};
use models::Asset;

mod cli;
mod client;
mod models;

fn main() {
    let mode = get_storage_mode();
    let (version, versions) = get_version(mode);
    let threads = get_thread_count();

    //clearscreen::clear();
    print!("\x1B[2J\x1B[1;1H");

    let assets = simple_spinner::<HashMap<String, Asset>>("receiving the asset list", || {
        get_assets(&get_asset_index_url(&version, &versions))
    });
}
