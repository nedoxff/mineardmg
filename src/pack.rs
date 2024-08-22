use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use cliclack::{progress_bar, spinner, MultiProgress};
use dashmap::DashMap;
use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read, Write},
};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};

use crate::{
    client::get_client_archive,
    models::{ClientPackVersion, ClientVersionInformation, PackInformationMetadata, PackMetadata},
};

pub fn determine_resource_pack_version(version_url: &str) -> Result<u32> {
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

pub fn write_resource_pack(
    pack_version: u32,
    gain: u32,
    lookup: &HashMap<String, String>,
    data: &DashMap<String, Bytes>,
) -> Result<()> {
    let file = File::create("output.zip").context("failed to create the output file")?;
    let mut zip = ZipWriter::new(file);

    let multi = MultiProgress::new("packing the output archive");

    let spinner = multi.add(spinner());
    spinner.start("writing pack.mcmeta");
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
    spinner.stop("wrote pack.mcmeta");

    let pb = multi.add(progress_bar(data.len() as u64));
    pb.start("writing files to the output archive");
    for entry in data {
        let (hash, bytes) = entry.pair();
        let path = "assets/".to_string()
            + lookup
                .get(hash)
                .ok_or(anyhow!("didn't find a path for asset with hash {}", &hash))?;

        zip.start_file(&path, SimpleFileOptions::default())
            .context(format!("failed to create an asset ({})", &path))?;
        zip.write_all(bytes)
            .context(format!("failed to write contents to an asset ({})", &path))?;

        pb.inc(1);
    }

    zip.finish()
        .context("failed to write the archive to disk")?;
    pb.stop("wrote files to the output archive");

    Ok(())
}
