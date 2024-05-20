SHELL = /bin/sh

ifeq ($(PREFIX),)
    PREFIX := /usr/local
endif
MANPREFIX := $(PREFIX)/share/man

help:
	@echo "Use one of the following options:"
	@echo "'make req' - Checks dependencies and environment"
	@echo "'make install' - Installs synchronice"
	@echo "'make uninstall' - Uninstalls synchronice"
	@echo "'make reinstall' - Reinstalls synchronice"

deps:
	@echo "Checking for dependencies..."
	@echo "Make sure you have 'ncurses' installed."

env:
	@echo "Looking for Rust compiler..."
ifneq ($(shell command -v cargo),)
	@echo "'cargo' found, build can continue."
else
	@echo "'cargo' not found!"
	@echo "Attemping to install 'cargo' using Crater..."
	git clone https://github.com/crater-space/cli /tmp/crater-cli
	/tmp/crater-cli/crater install cargo
	rm -rf /tmp/crater-cli
endif

req: deps env

clean:
	@echo "Cleaning build directory..."
	cargo clean
	@echo "Build directory cleaned"

build:
	@echo "Building project..."
	cargo build --release
	@echo "Build complete"

place:
	@echo "Installing binary..."
	sudo install ./target/release/synchronice $(PREFIX)/bin/
	@echo "Binary installed"

manpage:
	@echo "Creating manpage..."
	sudo mkdir -p $(MANPREFIX)/man1
	sudo cp ./man/synchronice.1 $(MANPREFIX)/man1/
	@echo "Manpage created"

install: req clean build place manpage
	@echo "synchronice is now installed!"

uninstall:
	@echo "Uninstalling synchronice..."
	sudo rm $(PREFIX)/bin/synchronice
	sudo rm $(MANPREFIX)/man1/synchronice.1
	@echo "Uninstallation was successful!"

reinstall: uninstall install
