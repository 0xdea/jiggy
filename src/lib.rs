#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/jiggy/master/.img/logo.png")]

use std::time::Duration;
use std::{process, thread};

use mouse_rs::Mouse;
use mouse_rs::types::Point;
use spinners::{Spinner, Spinners};

/// Empty enum to emulate the never type `!` and avoid using `#![feature(never_type)]`.
///
/// For additional information, see:
/// * <https://rust-reference.irust.net/en-us/types/never.html>
/// * <https://github.com/rust-lang/rust/issues/35121>
pub enum Void {}

/// Check the mouse position every `interval`; jiggle the mouse pointer and scroll the wheel if the
/// position hasn't changed.
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
        process::exit(0);
    })?;

    loop {
        let cur_position = mouse.get_position()?;
        if is_same_pos(&cur_position, &old_position) {
            jiggle_and_scroll(&mouse, &cur_position)?;
        }
        old_position = cur_position;
        thread::sleep(interval);
    }
}

/// Slightly jiggle the mouse pointer and scroll the mouse wheel.
fn jiggle_and_scroll(mouse: &Mouse, position: &Point) -> Result<(), Box<dyn std::error::Error>> {
    // Slightly jiggle the mouse pointer
    mouse.move_to(position.x + 1, position.y + 1)?;
    mouse.move_to(position.x, position.y)?;

    // Scroll the mouse wheel (a zero delta is apparently enough and has no side effects)
    mouse.wheel(0)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_pointer_goes_back_to_its_old_position() {
        // Arrange
        let m = Mouse::new();
        let p = m
            .get_position()
            .expect("failed to get initial mouse position");

        // Act
        jiggle_and_scroll(&m, &p).expect("unable to jiggle and scroll mouse");
        let q = m
            .get_position()
            .expect("failed to get final mouse position");

        // Assert
        assert_eq!(
            (p.x, p.y),
            (q.x, q.y),
            "mouse pointer didn't go back to its old position"
        );
    }
}
