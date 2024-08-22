use crate::client::get_versions;
use anyhow::{anyhow, Context, Result};
use cliclack::{input, select, spinner};

pub enum StorageMode {
    Offline,
    Online,
}

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

pub fn get_storage_mode() -> Result<StorageMode> {
    let mode = select("how would you like to create a resource pack?")
        .item(
            "online",
            "download the resources from any version (online)",
            "",
        )
        .item(
            "offline",
            "use an already installed version of minecraft (offline)",
            "",
        )
        .interact()?;

    if mode == "offline" {
        Ok(StorageMode::Offline)
    } else {
        Ok(StorageMode::Online)
    }
}

pub fn get_version(mode: StorageMode) -> Result<(String, String)> {
    match mode {
        StorageMode::Offline => panic!("offline mode is currently not supported."),
        StorageMode::Online => {
            let version_response =
                get_versions().context("failed to get available minecraft versions")?;

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

            let latest_release: String =
                format!("latest release ({})", &version_response.latest.release);
            let latest_snapshot: String =
                format!("latest snapshot ({})", &version_response.latest.snapshot);

            let version_option = select(
                "which version of minecraft would you like to create the resource pack for?",
            )
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
                    Ok((
                        selection.clone(),
                        version_response
                            .versions
                            .into_iter()
                            .find(|v| v.id == selection)
                            .ok_or(anyhow!("didn't find the url for the specified version"))?
                            .url,
                    ))
                }
                "view_snapshots" => {
                    let selection = select("which snapshot version would you like to use?")
                        .items(&snapshots)
                        .interact()
                        .context("failed to select a value (version_select_snapshot)")?;
                    Ok((
                        selection.clone(),
                        version_response
                            .versions
                            .into_iter()
                            .find(|v| v.id == selection)
                            .ok_or(anyhow!("didn't find the url for the specified version"))?
                            .url,
                    ))
                }
                _ => Ok((
                    version_option.clone(),
                    version_response
                        .versions
                        .into_iter()
                        .find(|v| v.id == version_option)
                        .ok_or(anyhow!("didn't find the url for the specified version"))?
                        .url,
                )),
            }
        }
    }
}

pub fn get_thread_count() -> Result<usize> {
    let selection = select("how many threads would you like to dedicate?")
        .item(1, "single-threaded (1)", "")
        .item(2, "2 threads", "")
        .item(4, "4 threads", "")
        .item(8, "8 threads", "")
        .item(0, "custom", "")
        .interact()?;

    if selection == 0 {
        input("enter the amount of threads to dedicate:")
            .validate(|inp: &String| {
                if inp.parse::<usize>().is_err() || inp.parse::<usize>().unwrap() == 0 {
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
