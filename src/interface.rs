//! Contains functions to create the text-based interface.

use cursive::views::{Dialog, DummyView, LinearLayout, Panel, TextView};
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

    // Add the global commands
    siv.add_global_callback('r', reload_config);
    siv.add_global_callback('s', refresh_connection_statuses);
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
    // Remove previous layer
    s.pop_layer();

    // Replace with new layer
    s.add_layer(get_updated_dashboard(true));
}

/// Refreshes the connection statuses.
///
/// # Example
///
/// ```
/// refresh_connection_statuses(s);
/// ```
pub fn refresh_connection_statuses(s: &mut Cursive) {
    // Remove previous layer
    s.pop_layer();

    // Replace with new layer
    s.add_layer(get_updated_dashboard(false));
}

/// Returns a new or updated dashboard layer.
///
/// # Example
///
/// ```
/// get_updated_dashboard(true)
/// ```
pub fn get_updated_dashboard(is_initial_load: bool) -> Dialog {
    // Get latest viewmodel
    let viewmodel = construct_viewmodel(is_initial_load);

    // Get display layouts
    let (folders_layout, devices_layout) = get_display_layouts(viewmodel);

    // Return latest dashboard layer
    get_dashboard_layer(folders_layout, devices_layout)
}

/// Constructs a new viewmodel.
///
/// # Example
///
/// ```
/// construct_viewmodel(true);
/// ```
pub fn construct_viewmodel(is_initial_load: bool) -> viewmodel::Viewmodel {
    // Get version and config
    let version = service::get_version();
    let config = service::get_config();

    // Get recent events
    let events = if !is_initial_load {
        service::get_events()
    } else {
        service::Events(vec![])
    };

    // Return the latest viewmodel
    viewmodel::get_updated_viewmodel(version, config, events)
}

/// Gets a tuple of data-filled layouts.
///
/// # Example
///
/// ```
/// get_display_layouts(viewmodel);
/// ```
pub fn get_display_layouts(viewmodel: viewmodel::Viewmodel) -> (LinearLayout, LinearLayout) {
    // Create folders and devices layouts
    let mut folders_layout: LinearLayout = LinearLayout::vertical();
    let mut devices_layout: LinearLayout = LinearLayout::vertical();

    // Populate list of folders
    folders_layout.add_child(Panel::new(TextView::new("Folder 1")));

    // Populate list of devices
    devices_layout.add_child(Panel::new(TextView::new("Device 1")));

    (folders_layout, devices_layout)
}

/// Creates an updated layer based on latest viewmodel.
///
/// # Example
///
/// ```
/// get_dashboard_layer(folders_layout, devices_layout);
/// ```
pub fn get_dashboard_layer(folders_layout: LinearLayout, devices_layout: LinearLayout) -> Dialog {
    Dialog::around(
        LinearLayout::vertical().child(DummyView).child(
            LinearLayout::horizontal()
                .child(Dialog::around(folders_layout).title("Folders"))
                .child(Dialog::around(devices_layout).title("Devices")),
        ),
    )
    .title("Synchronice")
    .button("(R)eload", reload_config)
    .button("Refre(S)h", refresh_connection_statuses)
    .button("(Q)uit", |s| s.quit())
}
