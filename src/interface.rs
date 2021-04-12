//! Contains functions to create the text-based interface.

use cursive::views::{Dialog, TextView};
use cursive::Cursive;

use crate::config;
use crate::service;
use crate::viewmodel;

/// Creates the text-based interface using curses.
///
/// # Example
///
/// ```
/// show_dashboard();
/// ```
pub fn show_dashboard() {
    // Create the cursive root
    let mut siv = cursive::default();

    // Add the global 'quit' command
    siv.add_global_callback('q', |s| s.quit());

    // Show notice about being under development
    show_development_notice(&mut siv);

    // Start the event loop
    siv.run();
}

/// Shows the note about being under development.
///
/// # Example
///
/// ```
/// show_development_notice();
/// ```
pub fn show_development_notice(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("Synchronice is still under development!")
            .title("Notice")
            .button("Proceed", start),
    );
}

/// Loads data and shows the dashboard.
///
/// # Example
///
/// ```
/// start();
/// ```
pub fn start(s: &mut Cursive) {
    s.pop_layer();

    let connection = config::get_connection();

    // Check for connection details
    if connection.apikey == "" || connection.address == "" {
        s.add_layer(
            Dialog::text("Could not fetch connection details!")
                .title("Error")
                .button("Quit", |s| s.quit()),
        );
    }

    // Get version and config
    let version = service::get_version();
    let config = service::get_config();

    // Get recent events
    let events = service::get_events();

    // Get updated viewmodel
    let viewmodel = viewmodel::get_updated_viewmodel(version, config, events);

    // // TODO: Implement

    // s.add_layer(
    //     Dialog::text(format!("{}", version.longVersion))
    //         .title("Test")
    //         .button("OK", |s| s.quit()),
    // );

    // s.add_layer(
    //     Dialog::text(format!("{}", config.devices[0].name))
    //         .title("Devices")
    //         .button("OK", |s| s.quit()),
    // );

    // s.add_layer(
    //     Dialog::text(format!("{}", events.0[0].id))
    //         .title("Events")
    //         .button("OK", |s| s.quit()),
    // );
}
