#![doc = env!("CARGO_PKG_DESCRIPTION")]
#![doc = ""]
#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/jiggy/master/.img/logo.png")]

use std::time::Duration;
use std::{error, process, thread};

use mouse_rs::Mouse;
use mouse_rs::types::Point;
use spinners::{Spinner, Spinners};

/// Checks the mouse position every `interval`; jiggles the mouse pointer and
/// scrolls the wheel if the position hasn't changed.
///
/// # Errors
///
/// Returns an error if the mouse position can't be read, the mouse can't be
/// moved or scrolled, or the Ctrl+C handler can't be installed.
#[expect(clippy::non_ascii_literal, reason = "this is fine 🔥")]
#[expect(
    clippy::exit,
    reason = "the process is terminated by the signal handler"
)]
pub fn run(interval: Duration) -> Result<(), Box<dyn error::Error>> {
    let mouse = Mouse::new();
    let mut old_position = mouse.get_position()?;

    println!("⏰  Just chillin' for {}s", interval.as_secs());
    let mut spinner = Spinner::new(Spinners::Moon, "Gettin' jiggy wit it!".to_owned());

    ctrlc::set_handler(move || {
        spinner.stop_with_message("✌️  Peace out!".to_owned());
        process::exit(0);
    })?;

    loop {
        let cur_position = mouse.get_position()?;
        if is_same_position(&cur_position, &old_position) {
            jiggle_and_scroll(&mouse, &cur_position)?;
        }
        old_position = cur_position;
        thread::sleep(interval);
    }
}

/// Returns `true` if `before` and `after` are the same position, i.e., the
/// mouse hasn't moved.
#[must_use]
const fn is_same_position(before: &Point, after: &Point) -> bool {
    before.x == after.x && before.y == after.y
}

/// Slightly jiggles the mouse pointer and scrolls the mouse wheel.
fn jiggle_and_scroll(mouse: &Mouse, position: &Point) -> Result<(), Box<dyn error::Error>> {
    // Slightly jiggle the mouse pointer.
    mouse.move_to(position.x.saturating_add(1), position.y.saturating_add(1))?;
    mouse.move_to(position.x, position.y)?;

    // Scroll the mouse wheel (a zero delta is apparently enough and has no side
    // effects).
    mouse.wheel(0)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unchanged_position_is_same_position() {
        let before = Point { x: 100, y: 200 };
        let after = Point { x: 100, y: 200 };

        assert!(
            is_same_position(&before, &after),
            "identical positions should be the same position"
        );
    }

    #[test]
    fn horizontal_move_is_not_same_position() {
        let before = Point { x: 100, y: 200 };
        let after = Point { x: 101, y: 200 };

        assert!(
            !is_same_position(&before, &after),
            "a move along x should not be the same position"
        );
    }

    #[test]
    fn vertical_move_is_not_same_position() {
        let before = Point { x: 100, y: 200 };
        let after = Point { x: 100, y: 201 };

        assert!(
            !is_same_position(&before, &after),
            "a move along y should not be the same position"
        );
    }

    #[test]
    #[expect(clippy::expect_used, reason = "tests can use `expect`")]
    fn mouse_pointer_goes_back_to_its_old_position() {
        // Arrange.
        let mouse = Mouse::new();
        let before = mouse
            .get_position()
            .expect("failed to get initial mouse position");

        // Act.
        jiggle_and_scroll(&mouse, &before).expect("unable to jiggle and scroll mouse");
        let after = mouse
            .get_position()
            .expect("failed to get final mouse position");

        // Assert.
        assert_eq!(
            (before.x, before.y),
            (after.x, after.y),
            "mouse pointer didn't go back to its old position"
        );
    }
}
