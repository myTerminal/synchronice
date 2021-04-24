//! Holds static data for cache.

use crate::service::{Config, Events};

// Holds config fetched from service
pub static mut CONFIG: Vec<Config> = vec![];

// Holds events fetched from service
pub static mut EVENTS: Vec<Events> = vec![];
