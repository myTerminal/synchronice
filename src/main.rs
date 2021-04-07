//! # synchronice
//!
//! `synchronice` is a text-based frontend for Syncthing.

#[macro_use]
extern crate serde_derive;

use std::process;

use ansi_term::Color;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, SubCommand};

mod config;
mod environment;
mod interface;
mod service;

use environment::does_exist;
use interface::show_dashboard;

/// The entry point to synchronice.
///
/// Checks if Syncthing is installed in the environmen and calls `run`.
/// Otherwise, ends the program with an appropriate message.
fn main() {
    // Detect presence of Syncthing
    if does_exist("syncthing") {
        // Run synchronice with input parameters
        run();
    } else {
        // Print error message about absence of Syncthing
        println!(
            "{}",
            Color::Red.paint("Syncthing doesn't seem to be installed!")
        );

        // Exit synchronice
        process::exit(0);
    }
}

/// Runs Synchronice.
fn run() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("launch").about("Launches a text-based interface for Syncthing"),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("launch") {
        // Start the curses-based interface
        show_dashboard();
    } else {
        // Ask to be run with a command
        println!(
            "{}",
            Color::Red.paint("Please run synchronice with a command!")
        );
    }
}
