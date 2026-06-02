# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Test (local)
cargo test

# Test (Linux headless, matches CI)
xvfb-run cargo test

# Lint
cargo clippy --all-targets   # warnings treated as errors
cargo fmt --all --check
```

## Architecture

**Jiggy** is a single-binary Rust crate — a cross-platform mouse jiggler that keeps systems awake by detecting idle mouse position and performing minimal mouse movements. Designed to prevent "away" status in apps like Microsoft Teams.

**Two source files:**
- `src/main.rs` — CLI entry point: parses the optional interval-in-seconds argument, prints package metadata, calls `jiggy::run()`
- `src/lib.rs` — Core logic: `run(interval)` event loop + private `jiggle_and_scroll()` helper

**`run(interval: Duration)`** polls mouse position on each tick. If the position is unchanged since the last tick, it calls `jiggle_and_scroll()`, which moves the cursor +1px and back, then sends a zero-delta wheel scroll. A Ctrl+C handler (via `ctrlc`) signals the loop to exit gracefully with a spinner message.

**Dependencies:** `mouse-rs` (cross-platform mouse control), `spinners` (terminal UX), `ctrlc` (signal handling).

## Platform Notes

- **Linux:** requires `libxdo-dev`; CI runs tests under `xvfb-run` for headless display
- **macOS:** terminal app needs Accessibility permissions
- **Windows:** no extra dependencies

## Lint Strictness

The project enforces ~20 `clippy::restriction` lints (including `unwrap_used`, `expect_used`, `panic`, `todo`). All public items require docs (`missing_docs`). Unsafe code requires a `// SAFETY:` comment. Avoid `unwrap()`/`expect()` — propagate errors with `?`.
