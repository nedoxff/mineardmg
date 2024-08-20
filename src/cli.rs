use crate::{
    client::get_versions,
    models::{Version, VersionListResponse},
};
use inquire::{Select, Text};
use std::{collections::HashMap, process::exit};

pub enum StorageMode {
    Offline,
    Online,
}

pub fn get_storage_mode() -> StorageMode {
    const OFFLINE_MODE: &str = "use an already installed version of minecraft (offline)";
    const ONLINE_MODE: &str = "download the resources from any version (online)";

    let mode: &str = Select::new(
        "how would you like to create a resource pack?",
        vec![OFFLINE_MODE, ONLINE_MODE],
    )
    .prompt()
    .expect("failed to select an option (start)");

    if mode == OFFLINE_MODE {
        StorageMode::Offline
    } else {
        StorageMode::Online
    }
}

pub fn get_version(mode: StorageMode) -> (String, VersionListResponse) {
    match mode {
        StorageMode::Offline => panic!("offline mode is currently not supported."),
        StorageMode::Online => {
            let version_response =
                get_versions().expect("failed to get available minecraft versions");
            let mut groups: HashMap<String, Vec<Version>> = HashMap::new();
            version_response
                .clone()
                .versions
                .into_iter()
                .for_each(|version| {
                    groups
                        .entry(version.variant.clone())
                        .or_default()
                        .push(version);
                });

            let latest_release: String =
                format!("latest release ({})", &version_response.latest.release);
            let latest_snapshot: String =
                format!("latest snapshot ({})", &version_response.latest.snapshot);
            const VIEW_RELEASES: &str = "view releases";
            const VIEW_SNAPSHOTS: &str = "view snapshots";

            let version_options = vec![
                latest_release.as_str(),
                latest_snapshot.as_str(),
                VIEW_RELEASES,
                VIEW_SNAPSHOTS,
            ];

            let version_option = Select::new(
                "which version of minecraft would you like to create the resource pack for?",
                version_options,
            )
            .prompt()
            .expect("failed to select an option (version_category_select)");

            if version_option == latest_release {
                (version_response.latest.release.clone(), version_response)
            } else if version_option == latest_snapshot {
                (version_response.latest.snapshot.clone(), version_response)
            } else {
                match version_option {
                    VIEW_RELEASES => (
                        Select::new(
                            "which release version would you like to use?",
                            groups
                                .entry("release".to_string())
                                .or_default()
                                .into_iter()
                                .map(|it| it.id.clone())
                                .collect(),
                        )
                        .prompt()
                        .expect("failed to select an option (version_select_release)"),
                        version_response,
                    ),
                    VIEW_SNAPSHOTS => (
                        Select::new(
                            "which snapshot version would you like to use?",
                            groups
                                .entry("snapshot".to_string())
                                .or_default()
                                .into_iter()
                                .map(|it| it.id.clone())
                                .collect(),
                        )
                        .prompt()
                        .expect("failed to select an option (version_select_snapshot)"),
                        version_response,
                    ),
                    _ => exit(1),
                }
            }
        }
        _ => exit(1),
    }
}

pub fn get_thread_count() -> i32 {
    let options = vec![
        "single-threaded (1)",
        "2 threads",
        "4 threads",
        "8 threads",
        "custom",
    ];
    let selection = Select::new("how many threads would you like to dedicate?", options)
        .prompt()
        .expect("failed to choose option (select_thread_count)");

    match selection {
        "single-threaded (1)" => 1,
        "2 threads" => 2,
        "4 threads" => 4,
        "8 threads" => 8,
        "custom" => Text::new("enter the amount of threads to dedicate:")
            .prompt()
            .expect("failed to enter text (select_custom_thread_count)")
            .parse()
            .unwrap(),
        _ => panic!(),
    }
}
