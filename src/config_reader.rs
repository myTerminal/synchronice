//! Reads Syncthing configuration file.

use std::fs;

extern crate quick_xml;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::environment;

/// Reads apikey for REST API.
///
/// # Example
///
/// ```
/// get_api_key();
/// ```
pub fn get_api_key() -> String {
    let config_file_contents = get_syncthing_config_as_string();

    let mut reader = Reader::from_str(&config_file_contents);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut apikey = String::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"apikey" => {
                apikey = reader
                    .read_text(b"apikey", &mut Vec::new())
                    .expect("Cannot decode text value");
            }
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }

    apikey
}

/// Returns Syncthing configuration as a string.
///
/// # Example
///
/// ```
/// get_syncthing_config_as_string();
/// ```
pub fn get_syncthing_config_as_string() -> String {
    fs::read_to_string(get_syncthing_config_path()).expect("Could not read Syncthing config file!")
}

/// Finds Syncthing configuration path.
///
/// # Example
///
/// ```
/// get_syncthing_config_path();
/// ```
pub fn get_syncthing_config_path() -> String {
    let username_as_list = environment::run_command_and_get_list("echo $USER");

    format!(
        "/home/{0}/.config/syncthing/config.xml",
        username_as_list[0]
    )
}
