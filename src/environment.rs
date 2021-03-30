//! Contains functions to interact with the environment.

use std::process::Command;

/// Runs a shell command with no stdin and returns the output as a list.
///
/// # Example
///
/// ```
/// run_command_and_get_list("ls -la");
/// ```
pub fn run_command_and_get_list(expression: &str) -> Vec<String> {
    // Run the expression as a shell command and obtain the output
    let output = Command::new("/bin/bash")
        .arg("-c")
        .arg(expression)
        .output()
        .expect("Something went wrong!");

    // Return the stdout as a string
    std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .to_string()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

/// Returns whether a command exists.
///
/// # Example
///
/// ```
/// does_exist("paru");
/// ```
pub fn does_exist(command: &str) -> bool {
    // Return true if the command exists in the environment
    run_command_and_get_list(&format!("which {}", command))[0] != ""
}
