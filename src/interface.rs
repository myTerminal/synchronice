//! Contains functions to create the text-based interface.

use cursive::views::{Dialog, TextView};
use cursive::Cursive;

use crate::config;

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

    // Check for apikey
    if config::get_api_key() == "" {
        s.add_layer(
            Dialog::text("Could not fetch API key!")
                .title("Error")
                .button("Quit", |s| s.quit()),
        );
    }

    // TODO: Implement
}
