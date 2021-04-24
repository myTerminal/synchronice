//! Connects to Syncthing's REST API.

use restson::{Error, RestClient, RestPath};

use crate::config;
use crate::types::Events;
use crate::types::{Config, Version};

/// An implementation for RestPath.
///
/// This is to implement the path for /rest/system/version.
impl RestPath<()> for Version {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("rest/system/version"))
    }
}

/// Returns the version for current Syncthing instance.
///
/// # Example
///
/// ```
/// get_version();
/// ```
pub fn get_version() -> Version {
    let connection = config::get_connection();

    let mut client = RestClient::new(&format!("http://{}", &connection.address)).unwrap();
    client
        .set_header("X-API-KEY", &connection.apikey)
        .expect("Couldn't set header for request");

    let data: Version = client.get(()).unwrap();

    data
}

/// An implementation for RestPath.
///
/// This is to implement the path for /rest/system/config.
impl RestPath<()> for Config {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("rest/system/config"))
    }
}

/// Returns the configuration for current Syncthing instance.
///
/// # Example
///
/// ```
/// get_config();
/// ```
pub fn get_config() -> Config {
    let connection = config::get_connection();

    let mut client = RestClient::new(&format!("http://{}", &connection.address)).unwrap();
    client
        .set_header("X-API-KEY", &connection.apikey)
        .expect("Couldn't set header for request");

    let data: Config = client.get(()).unwrap();

    data
}

/// An implementation for RestPath.
///
/// This is to implement the path for /rest/events.
impl RestPath<()> for Events {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("rest/events"))
    }
}

/// Returns the Events from Syncthing.
///
/// # Example
///
/// ```
/// get_events();
/// ```
pub fn get_events() -> Events {
    let connection = config::get_connection();

    let mut client = RestClient::new(&format!("http://{}", &connection.address)).unwrap();
    client
        .set_header("X-API-KEY", &connection.apikey)
        .expect("Couldn't set header for request");

    let query = vec![("events", "DeviceConnected,DeviceDisconnected")];
    let data: Events = client.get_with::<_, Events>((), &query).unwrap();

    data
}
