use anyhow::{Context, Result};
use std::io::{Cursor, Read};
use zip::ZipArchive;

use crate::{
    client::get_client_archive,
    models::{ClientPackVersion, ClientVersionInformation, Version},
};

pub fn determine_resource_pack_version(version_url: &str) -> Result<i32> {
    let client = get_client_archive(version_url)?;
    let mut zipfile =
        ZipArchive::new(Cursor::new(client)).context("failed to open the zip archive")?;
    let mut version_file = zipfile
        .by_name("version.json")
        .context("the zip archive does not contain a version.json file")?;

    let mut data = String::new();
    version_file
        .read_to_string(&mut data)
        .context("failed to read version.json from the archive")?;

    let client_information: ClientVersionInformation =
        serde_json::from_str(&data).context("failed to deserialize the version.json file")?;
    match client_information.pack_version {
        ClientPackVersion::Old(version) => Ok(version),
        ClientPackVersion::New { resource, .. } => Ok(resource),
    }
}

pub fn begin_resource_pack(version: &Version) {}
