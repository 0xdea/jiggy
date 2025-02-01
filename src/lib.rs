//!
//! jiggy - Minimalistic cross-platform mouse jiggler
//! Copyright (c) 2025 Marco Ivaldi <raptor@0xdeadbeef.info>
//!
//! > "If I wanted the government in my house I'd buy an Alexa."
//! >
//! > -- Rick Sanchez
//!
//! Jiggy is a minimalistic (but effective) cross-platform mouse jiggler written in Rust. It might
//! be useful for a number of "reasons".
//!
//! *Disclaimer: I'm not responsible for any problems that might arise due to using this program to
//! pretend you're "working from home".*
//!
//! ## Features
//! * Minimalistic (but effective) mouse jiggler, with no setup needed.
//! * As implemented, mouse jiggling won't interfere with your regular activities.
//! * Thanks to mouse wheel scroll, the new Microsoft Teams should not display you as away.
//! * Cross-platform support for macOS, Windows, and Linux.
//!
//! ## See also
//! * <https://github.com/arkane-systems/mousejiggler>
//! * <https://lib.rs/crates/jiggle>
//! * <https://lib.rs/crates/stayawake>
//! * <https://lib.rs/crates/meth>
//!
//! ## Installing
//! The easiest way to get the latest release is via [crates.io](https://crates.io/crates/jiggy):
//! ```sh
//! $ cargo install jiggy
//! ```
//! *Note: if run into problems building on Linux, you need to install `libxdo-dev`.*
//!
//! ## Compiling
//! Alternatively, you can build from [source](https://github.com/0xdea/jiggy).
//!
//! On macOS:
//! ```sh
//! $ git clone https://github.com/0xdea/jiggy
//! $ cd jiggy
//! $ cargo build --release
//! ```
//!
//! On Windows:
//! ```sh
//! $ git clone https://github.com/0xdea/jiggy
//! $ cd jiggy
//! $ cargo build --release
//!
//! TODO - linux (and maybe others) specific lib requirements (libxdo-devel and similar)
//! ```
//!
//! On Linux:
//! ```sh
//! $ git clone https://github.com/0xdea/jiggy
//! $ cd jiggy
//! $ cargo build --release
//!
//! TODO - linux (and maybe others) specific lib requirements (libxdo-devel and similar)
//! ```
//! *Note: if run into problems building on Linux, you need to install `libxdo-dev`.*
//!
//! ## Usage
//! ```sh
//! jiggy [check_interval_in_secs]
//! ```
//! *Note: on macOS, you might need to grant Accessibility privileges to your terminal application.*
//!
//! ## Tested on
//! * macOS Sequoia 15.2
//!
//! ## Changelog
//! * <https://github.com/0xdea/jiggy/blob/master/CHANGELOG.md>
//!

#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/jiggy/master/.img/logo.png")]

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use mouse_rs::types::Point;
use mouse_rs::Mouse;
use spinners::{Spinner, Spinners};

/// Check the mouse position every `interval` seconds; jiggle the mouse pointer and scroll the
/// wheel if the position hasn't changed.
pub fn run(interval: Duration) -> Result<(), Box<dyn std::error::Error>> {
    let mouse = Mouse::new();
    let mut old_position = mouse.get_position()?;
    let is_same_pos = |p1: &Point, p2: &Point| p1.x == p2.x && p1.y == p2.y;

    println!("⏰  Using check interval: {interval:?}");
    let mut sp = Spinner::new(Spinners::Moon, "Gettin' jiggy wit it!".into());

    ctrlc::set_handler(move || {
        sp.stop_with_message("👋  Goodbye!".into());
        exit(0);
    })?;

    loop {
        let cur_position = mouse.get_position()?;
        if is_same_pos(&cur_position, &old_position) {
            jiggle_and_scroll(&mouse, &cur_position)?;
        }
        old_position = cur_position;
        sleep(interval);
    }
}

/// Slightly jiggle the mouse pointer and scroll the mouse wheel.
fn jiggle_and_scroll(mouse: &Mouse, position: &Point) -> Result<(), Box<dyn std::error::Error>> {
    // Jiggle the mouse pointer
    mouse.move_to(position.x + 1, position.y + 1)?;
    mouse.move_to(position.x, position.y)?;

    // Scroll the mouse wheel
    mouse.wheel(1)?;
    mouse.wheel(-1)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_pointer_goes_back_to_its_old_position() {
        let m = Mouse::new();
        let p1 = m.get_position().unwrap();

        jiggle_and_scroll(&m, &p1).unwrap();

        let p2 = m.get_position().unwrap();

        assert!(p1.x == p2.x && p1.y == p2.y);
    }
}
