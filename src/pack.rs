use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{Cursor, Read, Write},
};
use zip::{read::ZipFile, write::SimpleFileOptions, ZipArchive, ZipWriter};

use crate::{
    client::get_client_archive,
    models::{
        ClientPackVersion, ClientVersionInformation, PackInformationMetadata, PackMetadata, Version,
    },
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

pub fn begin_resource_pack(pack_version: i32, gain: i32) -> Result<ZipWriter<File>> {
    let file = File::create("output.zip").context("failed to create the output file")?;
    let mut zip = ZipWriter::new(file);

    zip.start_file("pack.mcmeta", SimpleFileOptions::default())
        .context("failed to create pack.mcmeta")?;
    zip.write_all(
        serde_json::to_string(&PackMetadata {
            pack: PackInformationMetadata {
                pack_format: pack_version,
                description: format!("all vanilla sounds are increased by {}db.", gain),
            },
        })?
        .as_bytes(),
    )
    .context("failed to write contents to pack.mcmeta")?;

    Ok(zip)
}
