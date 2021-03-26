# synchronice

[![Crates.io version](https://img.shields.io/crates/v/synchronice)](https://crates.io/crates/synchronice)
[![Crates.io downloads](https://img.shields.io/crates/d/synchronice)](https://crates.io/crates/synchronice)  
[![Code Climate](https://codeclimate.com/github/myTerminal/synchronice.png)](https://codeclimate.com/github/myTerminal/synchronice)
[![Coverage Status](https://img.shields.io/coveralls/myTerminal/synchronice.svg)](https://coveralls.io/r/myTerminal/synchronice?branch=master)  
[![License](https://img.shields.io/github/license/myTerminal/synchronice.svg)](https://opensource.org/licenses/MIT)

A text-based frontend for Syncthing [in-progress]

## Background

[Syncthing](https://syncthing.net) is a great file-synchronization tool but it has a major limitation that it only has one user interface, and that is a graphical web-browser. For the ones among us spending most (or all) of our time within the terminal, starting an [X11 server](https://en.wikipedia.org/wiki/X_Window_System) (or any other [windowing system](https://en.wikipedia.org/wiki/Windowing_system) of your choice for that matter) just for Syncthing may not be practical.

*synchronice* aims to be a maturing text-based interface for Syncthing that utilizes Syncthing's REST API and doesn't require a graphical web-browser anymore. My plan for the project is to start with the most frequent interactions we have with Syncthing, slowly minimizing the need to use the web-app.

## Installation

There are a few different ways to get *synchronice*.

### As a binary crate using Cargo

If you already have [Cargo](https://github.com/rust-lang/cargo) installed, *synchronice* can be installed directly from [crates.io](https://crates.io) using the below command:

    cargo install synchronice

Once installed, in order to update and get the latest version, install it with `--force`:

    cargo install synchronice --force

Uninstalling is also as easy as:

    cargo uninstall synchronice

### As a native application package

#### Compile from source

    # Clone project to local
    git clone https://github.com/myTerminal/synchronice.git

    # Switch to project directory
    cd synchronice

    # Install with `make`
    make install

Uninstalling would need only a single command:

    make uninstall

Re-installation is also possible with:

    make reinstall

#### Through an existing package manager in your system

*synchronice* will soon be available to install from your operating system's package manager.

## How to Use

Simply run *synchronice* in your terminal to start a *curses* based text-interface and the rest should be self-explanatory.

### Further help with synchronice

To learn more about usage, refer to `manpage`:

    man synchronice

## External Dependency

Having a dependency on [cursive](https://crates.io/crates/cursive), synchronice also needs [ncurses](https://en.wikipedia.org/wiki/Ncurses) on the system to work. Until implemented as a part of the installation process, you might need to [install it manually](https://github.com/gyscos/cursive/wiki/Install-ncurses).
