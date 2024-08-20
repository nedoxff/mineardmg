use cli::{get_storage_mode, get_thread_count, get_version};
use client::get_asset_index_url;

mod cli;
mod client;
mod models;

fn main() {
    let mode = get_storage_mode();
    let (version, versions) = get_version(mode);
    let threads = get_thread_count();

    println!("{}", get_asset_index_url(&version, &versions.versions));
}
