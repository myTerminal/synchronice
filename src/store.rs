//! Holds static data for access across the application.

use crate::viewmodel;
use crate::service;

pub static mut STORE: Vec<viewmodel::Viewmodel> = vec![];

// TODO: Remove soon
pub static mut CONFIG: Vec<service::Config> = vec![];

/// Inits an empty store.
///
/// # Example
///
/// ```
/// init();
/// ```
pub fn init() {
    unsafe {
        STORE.push(viewmodel::Viewmodel {
            info: viewmodel::Info {
                version: "".to_string(),
                status: "".to_string(),
            },
            synced_folders: vec![],
            synced_devices: vec![],
        })
    }
}
