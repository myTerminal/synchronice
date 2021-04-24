//! Holds static data for access across the application.

use crate::service::{Config, Events, Version};

pub static mut STORE: Vec<Viewmodel> = vec![];

/// An abstract representation of info to be displayed on the dashboard.
///
/// This struct defines the info portion of the viewmodel.
pub struct Info {
    pub version: String,
    pub status: String,
}

/// An abstract representation of folders to be displayed on the dashboard.
///
/// This struct defines the folders portion of the viewmodel.
pub struct SyncedFolder {
    pub label: &'static str,
    pub path: &'static str,
    pub status: &'static str,
    pub devices: Vec<&'static str>,
}

/// An abstract representation of devices to be displayed on the dashboard.
///
/// This struct defines the devices portion of the viewmodel.
pub struct SyncedDevice {
    pub id: &'static str,
    pub name: &'static str,
    pub status: &'static str,
    pub folders: Vec<&'static str>,
}

/// An abstract representation of a viewmodel.
///
/// This struct defines the viewmodel for the entire dashboard.
pub struct Viewmodel {
    pub info: Info,
    pub synced_folders: Vec<SyncedFolder>,
    pub synced_devices: Vec<SyncedDevice>,
}

/// Inits an empty store.
///
/// # Example
///
/// ```
/// init();
/// ```
pub fn init() {
    unsafe {
        STORE.push(Viewmodel {
            info: Info {
                version: "".to_string(),
                status: "".to_string(),
            },
            synced_folders: vec![],
            synced_devices: vec![],
        })
    }
}

/// Gets updated viewmodel to be rendered on the interface.
///
/// # Example
///
/// ```
/// get_updated_viewmodel(v, c, e);
/// ```
pub fn get_updated_viewmodel(
    version: Version,
    config: &'static Config,
    events: Events,
) -> Viewmodel {
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
    return Viewmodel {
        info: Info {
            version: version.longVersion,
            status: "Connected".to_string(),
        },
        synced_folders,
        synced_devices,
    };
}
