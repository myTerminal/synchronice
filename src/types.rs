//! Contains all types used within the application.

/// An abtract representation of set of connection details.
///
/// This struct defines basic connection details for the Syncthing REST API.
pub struct Connection {
    pub apikey: String,
    pub address: String,
}

/// An abtract representation of response for /rest/system/version.
///
/// This struct defines a usable subset of the response.
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Version {
    pub longVersion: String,
}

/// An abstract representation of an associated device.
///
/// This struct defines a few properties of an associated device.
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AssociatedDevice {
    pub deviceID: String,
}

/// An abstract representation of a folder.
///
/// This struct defines a few properties of interests for a folder.
#[derive(Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
    pub label: String,
    pub path: String,
    pub devices: Vec<AssociatedDevice>,
}

/// An abstract representation of a device.
///
/// This struct defines a few properties of a device.
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Device {
    pub deviceID: String,
    pub name: String,
}

/// An abstract representation of Syncthing config.
///
/// This struct defines a few properties of Syncthing config.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub folders: Vec<Folder>,
    pub devices: Vec<Device>,
}

/// An abstract representation of a connected/disconnected device.
///
/// This struct defines a few properties of a connected/disconnected device.
#[derive(Serialize, Deserialize)]
pub struct ConnectionDevice {
    pub id: String,
}

/// An abstract representation of a connection event.
///
/// This struct defines a few properties of a connection event.
#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: usize,
    pub data: ConnectionDevice,
}

/// An abstract representation of a collection of events.
///
/// This struct defines a a connection events.
#[derive(Serialize, Deserialize)]
pub struct Events(pub Vec<Event>);

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
