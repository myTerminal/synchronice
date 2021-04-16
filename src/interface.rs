//! Contains functions to create the text-based interface.

use cursive::views::{Dialog, LinearLayout, Panel, TextView};
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
    siv.add_global_callback('r', reload_config);

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

    // Load interface
    reload_config(s);
}

/// Refreshes the list of folders and devices.
///
/// # Example
///
/// ```
/// reload_config(s);
/// ```
pub fn reload_config(s: &mut Cursive) {
    // Get version and config
    let version = service::get_version();
    let config = service::get_config();

    // Get recent events
    let events = service::Events(vec![]); // service::get_events();

    // Get updated viewmodel
    let viewmodel = viewmodel::get_updated_viewmodel(version, config, events);

    // Create folders and devices layouts
    let mut folders_layout: LinearLayout = LinearLayout::vertical();
    let mut devices_layout: LinearLayout = LinearLayout::vertical();

    // Populate list of folders
    folders_layout.add_child(Panel::new(TextView::new("Folder 1")));

    // Populate list of devices
    devices_layout.add_child(Panel::new(TextView::new("Devices 1")));

    // Construct the layer
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(Dialog::around(folders_layout).title("Folders"))
                .child(Dialog::around(devices_layout).title("Devices")),
        )
        .title("Synchronice")
        .button("(R)efresh", reload_config)
        .button("(Q)uit", |s| s.quit()),
    );

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
