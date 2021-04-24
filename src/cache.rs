//! Holds static data for cache.

use crate::service::{Config, Events};

// Holds config fetched from service
static mut CONFIGS: Vec<Config> = vec![];

/// Stores an instance of configs.
///
/// # Example
///
/// ```
/// set_config(config);
/// ```
pub fn set_config(config: Config) {
    unsafe {
        if CONFIGS.len() == 0 {
            CONFIGS.push(config);
        } else {
            CONFIGS[0] = config;
        }
    }
}

/// Retrieves stored configs.
///
/// # Example
///
/// ```
/// let config = get_config();
/// ```
pub fn get_config() -> &'static Config {
    unsafe {
        if CONFIGS.len() == 0 {
            set_config(Config {
                folders: vec![],
                devices: vec![],
            });
        }

        &CONFIGS[0]
    }
}

// Holds events fetched from service
static mut EVENTS: Vec<Events> = vec![];

/// Stores a collection of events.
///
/// # Example
///
/// ```
/// set_events(events);
/// ```
pub fn set_events(events: Events) {
    unsafe {
        if EVENTS.len() == 0 {
            EVENTS.push(events);
        } else {
            EVENTS[0] = events;
        }
    }
}

/// Retrieves a collection of cached events.
///
/// # Example
///
/// ```
/// let events = get_events();
/// ```
pub fn get_events() -> &'static Events {
    unsafe {
        if EVENTS.len() == 0 {
            set_events(Events(vec![]));
        }

        &EVENTS[0]
    }
}
