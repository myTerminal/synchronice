//! Contains datamodels and functions for rendering.

use crate::service::{Config, Events, Version};

/// An abstract representation of info to be displayed on the dashboard
///
/// This struct defines the info portion of the viewmodel.
pub struct Info {
    pub version: String,
    pub status: String,
}

/// An abstract representation of folders to be displayed on the dashboard
///
/// This struct defines the folders portion of the viewmodel.
pub struct SyncedFolders {
    pub label: String,
    pub path: String,
    pub status: String,
    pub devices: Vec<String>,
}

/// An abstract representation of devices to be displayed on the dasbboard
///
/// This struct defines the devices portion of the viewmodel.
pub struct SyncedDevices {
    pub id: String,
    pub name: String,
    pub folders: Vec<String>,
}

pub struct Viewmodel {
    info: Info,
    synced_folders: Vec<SyncedFolders>,
    synced_devices: Vec<SyncedDevices>,
}

// Gets updated viewmodel to be rendered on the interface.
pub fn get_updated_viewmodel(version: Version, config: Config, events: Events) -> Viewmodel {
    return Viewmodel {
        info: Info {
            version: version.longVersion,
            status: String::from("Connected"),
        },
        synced_folders: Vec::new(),
        synced_devices: Vec::new(),
    };
}
