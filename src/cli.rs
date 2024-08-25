use crate::client::get_versions;
use anyhow::{anyhow, bail, Context, Result};
use cliclack::{input, select, spinner};
use rfd::FileDialog;
use std::{env, path::PathBuf};

pub fn simple_spinner<T>(start_message: &str, stop_message: &str, func: impl Fn() -> T) -> T {
    let sp = spinner();
    sp.start(start_message);
    let result = func();
    sp.stop(stop_message);
    result
}

pub fn advanced_simple_spinner<T>(
    start_message: &str,
    stop_message: impl Fn(&T) -> String,
    func: impl Fn() -> Result<T>,
) -> Result<T> {
    let sp = spinner();
    sp.start(start_message);
    let result = func()?;
    sp.stop(stop_message(&result));
    Ok(result)
}

pub fn get_version_url() -> Result<String> {
    let version_response = get_versions().context("failed to get available minecraft versions")?;

    let releases = version_response
        .versions
        .iter()
        .filter_map(|v| {
            (v.variant == "release").then_some((v.id.clone(), v.id.clone(), String::new()))
        })
        .collect::<Vec<(String, String, String)>>();
    let snapshots = version_response
        .versions
        .iter()
        .filter_map(|v| {
            (v.variant == "snapshot").then_some((v.id.clone(), v.id.clone(), String::new()))
        })
        .collect::<Vec<(String, String, String)>>();

    let latest_release: String = format!("latest release ({})", &version_response.latest.release);
    let latest_snapshot: String =
        format!("latest snapshot ({})", &version_response.latest.snapshot);

    let version_option =
        select("which version of minecraft would you like to create the resource pack for?")
            .item(version_response.latest.release.clone(), latest_release, "")
            .item(
                version_response.latest.snapshot.clone(),
                latest_snapshot,
                "",
            )
            .item("view_releases".to_string(), "view releases", "")
            .item("view_snapshots".to_string(), "view snapshots", "")
            .interact()?;

    match version_option.as_str() {
        "view_releases" => {
            let selection = select("which release version would you like to use?")
                .items(&releases)
                .interact()
                .context("failed to select a value (version_select_release)")?;
            Ok(version_response
                .versions
                .into_iter()
                .find(|v| v.id == selection)
                .ok_or(anyhow!("didn't find the url for the specified version"))?
                .url)
        }
        "view_snapshots" => {
            let selection = select("which snapshot version would you like to use?")
                .items(&snapshots)
                .interact()
                .context("failed to select a value (version_select_snapshot)")?;
            Ok(version_response
                .versions
                .into_iter()
                .find(|v| v.id == selection)
                .ok_or(anyhow!("didn't find the url for the specified version"))?
                .url)
        }
        _ => Ok(version_response
            .versions
            .into_iter()
            .find(|v| v.id == version_option)
            .ok_or(anyhow!("didn't find the url for the specified version"))?
            .url),
    }
}

pub fn get_location() -> Result<PathBuf> {
    let selection = select("where would you like to save the resulting resouce pack?")
        .item("current", "current directory", "")
        .item("choose", "let me choose", "will open a file dialog")
        .interact()?;

    match selection {
        "current" => Ok(env::current_dir()?),
        "choose" => Ok(FileDialog::new()
            .add_filter("zip archive", &["zip"])
            .set_directory(env::current_dir()?)
            .save_file()
            .context("file dialog was canceled")
            .map_err(anyhow::Error::from)?),
        _ => bail!("invalid selection"),
    }
}

pub fn get_gain() -> Result<u32> {
    let selection = select("by how much would you like to increase the sounds?")
        .item(12, "12db", "")
        .item(24, "24db", "")
        .item(36, "36db", "")
        .item(48, "48db", "very dangerous")
        .item(
            72,
            "72db",
            "extremely dangerous. the sounds are bearly audible",
        )
        .item(0, "custom", "be careful")
        .interact()?;

    if selection == 0 {
        input("enter the amount of decibels to add:")
            .validate(|inp: &String| {
                if inp.parse::<i32>().is_err() || inp.parse::<i32>().unwrap() == 0 {
                    Err("please enter a non-zero number!")
                } else {
                    Ok(())
                }
            })
            .interact()
            .map_err(|err| err.into())
    } else {
        Ok(selection)
    }
}
