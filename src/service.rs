//! Connects to Syncthing's REST API

use restson::{Error, RestClient, RestPath};

use crate::config;

/// An abtract representation of response for /rest/system/version
///
/// This struct defines a usable subset of the response.
#[derive(Serialize, Deserialize)]
pub struct Version {
    pub longVersion: String,
}

/// An implementation for RestPath
///
/// This struct defines the path for /rest/system/version.
impl RestPath<()> for Version {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("rest/system/version"))
    }
}

/// Returns the version for current Syncthing instance
///
/// # Example
///
/// ```
/// get_version();
/// ```
pub fn get_version() -> Version {
    let connection = config::get_connection();

    let mut client = RestClient::new(&format!("http://{}", &connection.address)).unwrap();
    client.set_header("X-API-KEY", &connection.apikey);

    let data: Version = client.get(()).unwrap();

    data
}

/// An abstract representation of an associated device
///
/// This struct defines a few properties of an associated device.
#[derive(Serialize, Deserialize)]
pub struct AssociatedDevice {
    pub deviceID: String,
}

/// An abstract representation of a folder
///
/// This struct defines a few properties of interests for a folder.
#[derive(Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
    pub label: String,
    pub path: String,
    pub devices: Vec<AssociatedDevice>,
}

/// An abstract representation of a device
///
/// This struct defines a few properties of a device.
#[derive(Serialize, Deserialize)]
pub struct Device {
    pub deviceID: String,
    pub name: String,
}

/// An abstract representation of Syncthing config
///
/// This struct defines a few properties of Syncthing config.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub folders: Vec<Folder>,
    pub devices: Vec<Device>,
}

/// An implementation for RestPath
///
/// This struct defines the path for /rest/system/config.
impl RestPath<()> for Config {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("rest/system/config"))
    }
}

/// Returns the configuration for current Syncthing instance
///
/// # Example
///
/// ```
/// get_config();
/// ```
pub fn get_config() -> Config {
    let connection = config::get_connection();

    let mut client = RestClient::new(&format!("http://{}", &connection.address)).unwrap();
    client.set_header("X-API-KEY", &connection.apikey);

    let data: Config = client.get(()).unwrap();

    data
}

/// An abstract representation of a connected/disconnected device
///
/// This struct defines a few properties of a connected/disconnected device.
#[derive(Serialize, Deserialize)]
pub struct ConnectionDevice {
    pub id: String,
}

/// An abstract representation of a connection event
///
/// This struct defines a few properties of a connection event.
#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: usize,
    pub data: ConnectionDevice,
}

/// An abstract representation of a collection of events
///
/// This struct defines a a connection events.
#[derive(Serialize, Deserialize)]
pub struct Events(pub Vec<Event>);

/// An implementation for RestPath
///
/// This struct defines the path for /rest/events.
impl RestPath<()> for Events {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("rest/events"))
    }
}

/// Returns the Events from Syncthing
///
/// # Example
///
/// ```
/// get_events();
/// ```
pub fn get_events() -> Events {
    let connection = config::get_connection();

    let mut client = RestClient::new(&format!("http://{}", &connection.address)).unwrap();
    client.set_header("X-API-KEY", &connection.apikey);

    let query = vec![("events", "DeviceConnected,DeviceDisconnected")];
    let data: Events = client.get_with::<_, Events>((), &query).unwrap();

    data
}
