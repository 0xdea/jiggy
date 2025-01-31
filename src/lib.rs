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
//! * TODO - minimalistic, simple setup, teams/wheel, cross-platform
//!
//! ## See also
//! * TODO
//!
//! ## Installing
//! The easiest way to get the latest release is via [crates.io](https://crates.io/crates/jiggy):
//! ```sh
//! TODO
//! ```
//!
//! ## Compiling
//! Alternatively, you can build from [source](https://github.com/0xdea/jiggy):
//! ```sh
//! TODO
//! ```
//!
//! ## Usage
//! ```sh
//! TODO
//! ```
//!
//! ## Examples
//! TODO:
//! ```sh
//! TODO
//! ```
//!
//! TODO:
//! ```sh
//! TODO
//! ```
//!
//! ## Tested on/with
//! * TODO
//!
//! ## Changelog
//! * <https://github.com/0xdea/jiggy/blob/master/CHANGELOG.md>
//!
//! ## TODO
//! * TODO
//!

#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/jiggy/master/.img/logo.png")]

use std::thread;
use std::time::Duration;

use mouse_rs::types::Point;
use mouse_rs::Mouse;
use thread::sleep;

/// Check mouse position every `interval` seconds; jiggle the mouse pointer and scroll the wheel
/// if the position hasn't changed.
pub fn run(interval: Duration) -> Result<(), Box<dyn std::error::Error>> {
    let mouse = Mouse::new();
    let mut old_position = mouse.get_position()?;
    let is_same_pos = |p1: &Point, p2: &Point| p1.x == p2.x && p1.y == p2.y;

    println!("[*] Using check interval: {interval:?}");

    loop {
        let cur_position = mouse.get_position()?;
        if is_same_pos(&cur_position, &old_position) {
            jiggle_and_scroll(&mouse, &cur_position)?;
        }
        old_position = cur_position;
        sleep(interval);
    }
}

/// Slightly jiggle the mouse pointer and scroll the mouse wheel. Return any errors.
fn jiggle_and_scroll(mouse: &Mouse, position: &Point) -> Result<(), Box<dyn std::error::Error>> {
    // Jiggle the mouse pointer
    mouse.move_to(position.x + 1, position.y + 1)?;
    mouse.move_to(position.x, position.y)?;

    // Scroll the mouse wheel
    mouse.wheel(-1)?;
    mouse.wheel(1)?;

    Ok(())
}

// TODO
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
