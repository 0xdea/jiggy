//!
//! jiggler - Minimalistic cross-platform mouse jiggler
//! Copyright (c) 2025 Marco Ivaldi <raptor@0xdeadbeef.info>
//!
//! > "If I wanted the government in my house I'd buy an Alexa."
//! >
//! > -- Rick Sanchez
//!
//! TODO
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
//! The easiest way to get the latest release is via [crates.io](https://crates.io/crates/jiggler):
//! ```sh
//! TODO
//! ```
//!
//! ## Compiling
//! Alternatively, you can build from [source](https://github.com/0xdea/jiggler):
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
//! * <https://github.com/0xdea/jiggler/blob/master/CHANGELOG.md>
//!
//! ## TODO
//! * TODO
//!

#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/jiggler/master/.img/logo.png")]

use std::thread;
use std::time::Duration;

use mouse_rs::types::Point;
use mouse_rs::Mouse;

/// TODO Dispatch to function implementing the selected action
pub fn run(interval: Duration) -> anyhow::Result<()> {
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
        mouse
            .move_to(new_pos.x, new_pos.y)
            .expect("Failed to move cursor");
        mouse
            .move_to(old_pos.x, old_pos.y)
            .expect("Failed to move cursor");
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
