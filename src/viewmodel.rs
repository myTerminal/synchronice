//! Holds static data for access across the application.

use crate::cache;
use crate::types::{Config, Events, Info, SyncedDevice, SyncedFolder, Version, Viewmodel};
use crate::service;

/// Inits an empty store.
///
/// # Example
///
/// ```
/// init();
/// ```
pub fn init() {
    cache::set_viewmodel(Viewmodel {
        info: Info {
            version: "".to_string(),
            status: "".to_string(),
        },
        synced_folders: vec![],
        synced_devices: vec![],
    });
}

/// Gets the viewmodel in its current state.
///
/// # Example
///
/// ```
/// viewmodel = get_viewmodel();
/// ```
pub fn get_data() -> &'static Viewmodel {
    cache::get_viewmodel()
}

/// Constructs a new viewmodel.
///
/// # Example
///
/// ```
///  refresh_viewmodel(true);
/// ```
pub fn refresh_viewmodel(is_initial_load: bool) {
    // Get version and config
    let version = service::get_version();

    // Cache current config
    cache::set_config(service::get_config());

    // Get recent events
    let events = if !is_initial_load {
        service::get_events()
    } else {
        Events(vec![])
    };

    // Cache the latest viewmodel
    update_viewmodel(version, cache::get_config(), events);
}

/// Gets updated viewmodel to be rendered on the interface.
///
/// # Example
///
/// ```
/// update_viewmodel(v, c, e);
/// ```
pub fn update_viewmodel(version: Version, config: &'static Config, events: Events) {
    let synced_folders = config
        .folders
        .iter()
        .map(|f| SyncedFolder {
            label: &f.label,
            path: &f.path,
            status: "Unknown",
            devices: f
                .devices
                .iter()
                .map(|d| d.deviceID.as_str())
                .collect::<Vec<&'static str>>(),
        })
        .collect::<Vec<SyncedFolder>>();

    let synced_devices = config
        .devices
        .iter()
        .map(|d| SyncedDevice {
            id: &d.deviceID,
            name: &d.name,
            status: "Unknown",
            folders: vec![],
        })
        .collect::<Vec<SyncedDevice>>();

    // TODO: Implement
    cache::set_viewmodel(Viewmodel {
        info: Info {
            version: version.longVersion,
            status: "Connected".to_string(),
        },
        synced_folders,
        synced_devices,
    });
}
