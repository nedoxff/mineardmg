use cliclack::{input, select, spinner};

use crate::{client::get_versions, models::Version};

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

pub fn get_storage_mode() -> StorageMode {
    let mode = select("how would you like to create a resource pack?")
        .item(
            "offline",
            "use an already installed version of minecraft (offline)",
            "",
        )
        .item(
            "online",
            "download the resources from any version (online)",
            "",
        )
        .interact()
        .expect("failed to select a value (select_mode)");

    if mode == "offline" {
        StorageMode::Offline
    } else {
        StorageMode::Online
    }
}

pub fn get_version(mode: StorageMode) -> (String, Vec<Version>) {
    match mode {
        StorageMode::Offline => panic!("offline mode is currently not supported."),
        StorageMode::Online => {
            let version_response =
                get_versions().expect("failed to get available minecraft versions");

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
            .interact()
            .expect("failed to select a value (version_category_select)");

            match version_option.as_str() {
                "view_releases" => (
                    select("which release version would you like to use?")
                        .items(&releases)
                        .interact()
                        .expect("failed to select a value (version_select_release)"),
                    version_response.versions,
                ),
                "view_snapshots" => (
                    select("which snapshot version would you like to use?")
                        .items(&snapshots)
                        .interact()
                        .expect("failed to select a value (version_select_snapshot)"),
                    version_response.versions,
                ),
                _ => (version_option, version_response.versions),
            }
        }
    }
}

pub fn get_thread_count() -> usize {
    let selection = select("how many threads would you like to dedicate?")
        .item(1, "single-threaded (1)", "")
        .item(2, "2 threads", "")
        .item(4, "4 threads", "")
        .item(8, "8 threads", "")
        .item(0, "custom", "")
        .interact()
        .expect("failed to select a value (select_thread_count)");

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
            .expect("failed to enter a value (enter_custom_thread_count)")
    } else {
        selection
    }
}
