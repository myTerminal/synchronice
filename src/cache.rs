//! Holds static data for cache.

use crate::service::Config;

// Holds config fetched from service
pub static mut CONFIG: Vec<Config> = vec![];
