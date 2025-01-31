//!
//! jiggy - Minimalistic cross-platform mouse jiggler
//! Copyright (c) 2025 Marco Ivaldi <raptor@0xdeadbeef.info>
//!
//! > "If I wanted the government in my house I'd buy an Alexa."
//! >
//! > -- Rick Sanchez
//!
//! Jiggy is a minimalistic cross-platform mouse jiggler written in Rust. This may be useful for "reasons".
//!
//! *Disclaimer: I'm not responsible for any problems that might arise due to using this program to pretend you're "working
//! from home".*
//!
//! ## Features
//! * TODO - minimalistic, simple setup, cross-platform
//!
//! ## Blog post
//! * TODO
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

/// TODO Dispatch to function implementing the selected action
pub fn run(interval: Duration) -> Result<(), Box<dyn std::error::Error>> {
    // TODO
    println!("Duration: {interval:?}");

    let mouse = Mouse::new();

    loop {
        let old_pos = mouse.get_position().expect("Failed to get position");
        // dbg!(&old_pos);
        let new_pos = Point {
            x: old_pos.x + 1,
            y: old_pos.y + 1,
        };
        mouse.move_to(new_pos.x, new_pos.y)?;
        mouse.move_to(old_pos.x, old_pos.y)?;
        mouse.wheel(-1)?;
        mouse.wheel(1)?;
        thread::sleep(interval);
    }

    Ok(())
}

// Other functions ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
