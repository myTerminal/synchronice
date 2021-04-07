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

/// An implementation for 'Version'
///
/// This struct defines the path for /rest/system/version.
impl RestPath<()> for Version {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("rest/system/version"))
    }
}

pub fn get_version() -> Version {
    let connection = config::get_connection();

    let mut client = RestClient::new(&format!("http://{}", &connection.address)).unwrap();
    client.set_header("X-API-KEY", &connection.apikey);

    let data: Version = client.get(()).unwrap();

    data
}
