//! Contains datamodels and functions for rendering.

use crate::service::{Config, Events, Version};

/// An abstract representation of info to be displayed on the dashboard.
///
/// This struct defines the info portion of the viewmodel.
pub struct Info {
    pub version: String,
    pub status: &'static str,
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
    pub folders: Vec<&'static str>,
}

pub struct Viewmodel {
    pub info: Info,
    pub synced_folders: Vec<SyncedFolder>,
    pub synced_devices: Vec<SyncedDevice>,
}

// Gets updated viewmodel to be rendered on the interface.
pub fn get_updated_viewmodel(version: Version, config: Config, events: Events) -> Viewmodel {
    // TODO: Implement
    return Viewmodel {
        info: Info {
            version: version.longVersion,
            status: "Connected",
        },
        synced_folders: vec![SyncedFolder {
            label: "workspace",
            path: "~/_store/workspace",
            status: "Connected",
            devices: vec!["123-123-123-123"],
        }],
        synced_devices: vec![SyncedDevice {
            id: "123-123-123-123",
            name: "That-Device",
            folders: vec!["workspace"],
        }],
    };
}
