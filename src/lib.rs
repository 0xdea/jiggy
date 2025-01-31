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

use std::time::Duration;

// const NAME: type = ...;

// static NAME: type = ...;

/// Dispatch to function implementing the selected action
pub fn run(interval: Duration) -> anyhow::Result<()> {
    println!("Duration: {interval:?}");
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
