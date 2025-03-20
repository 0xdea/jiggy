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
//! * Thanks to non-invasive mouse wheel scrolling, the new Microsoft Teams should not display you as away.
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
//! cargo install jiggy
//! ```
//! *Note: if run into problems building on Linux, you need to install `libxdo-dev` or equivalent package.*
//!
//! ## Compiling
//! Alternatively, you can build from [source](https://github.com/0xdea/jiggy):
//! ```sh
//! git clone https://github.com/0xdea/jiggy
//! cd jiggy
//! cargo build --release
//! ```
//! *Note: if run into problems building on Linux, you need to install `libxdo-dev` or equivalent package.*
//!
//! ## Usage
//! Run jiggy as follows:
//! ```sh
//! jiggy [check_interval_in_secs]
//! ```
//! *Note: on macOS, you might need to grant Accessibility privileges to your terminal application.*
//!
//! ## Tested on
//! * Apple macOS Sequoia 15.2
//! * Ubuntu Linux 24.04.1 LTS
//! * Microsoft Windows 11 23H2
//!
//! ## Changelog
//! * <https://github.com/0xdea/jiggy/blob/master/CHANGELOG.md>
//!

#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/jiggy/master/.img/logo.png")]

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use mouse_rs::Mouse;
use mouse_rs::types::Point;
use spinners::{Spinner, Spinners};

/// Empty enum to emulate the never type `!` and avoid using `#![feature(never_type)]`.
///
/// For additional information, see:
/// * <https://rust-reference.irust.net/en-us/types/never.html>
/// * <https://github.com/rust-lang/rust/issues/35121>
pub enum Void {}

/// Check the mouse position every `interval` seconds; jiggle the mouse pointer and scroll the
/// wheel if the position hasn't changed.
///
/// ## Errors
///
/// Returns a generic error in case something goes wrong.
pub fn run(interval: Duration) -> Result<Void, Box<dyn std::error::Error>> {
    let mouse = Mouse::new();
    let mut old_position = mouse.get_position()?;
    let is_same_pos = |p: &Point, q: &Point| p.x == q.x && p.y == q.y;

    println!("⏰  Just chillin' for {interval:?}");
    let mut sp = Spinner::new(Spinners::Moon, "Gettin' jiggy wit it!".into());

    ctrlc::set_handler(move || {
        sp.stop_with_message("✌️  Peace out!".into());
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
    // Sightly jiggle the mouse pointer
    mouse.move_to(position.x + 1, position.y + 1)?;
    mouse.move_to(position.x, position.y)?;

    // Scroll the mouse wheel (a zero delta is apparently sufficient and has no side effects)
    mouse.wheel(0)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_pointer_goes_back_to_its_old_position() {
        let m = Mouse::new();
        let p = m.get_position().unwrap();

        jiggle_and_scroll(&m, &p).unwrap();

        let q = m.get_position().unwrap();

        assert!(
            p.x == q.x && p.y == q.y,
            "mouse pointer didn't go back to its old position"
        );
    }
}
