# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build --locked
cargo build --release --locked

# Test (local)
cargo test --locked

# Test (Linux headless, matches CI)
xvfb-run cargo test --locked

# Lint
cargo clippy --all-targets --locked -- -D warnings
cargo fmt --all --check

# Doc (warnings treated as errors)
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked

# Dependency vulnerability audit
cargo audit
```

## Architecture

**Jiggy** is a single-binary Rust crate — a cross-platform mouse jiggler that keeps systems awake by detecting idle mouse position and performing minimal mouse movements. Designed to prevent "away" status in apps like Microsoft Teams.

**Two source files:**

- `src/main.rs` — CLI entry point: parses the optional interval-in-seconds argument, prints package metadata, calls `jiggy::run()`
- `src/lib.rs` — Core logic: `run(interval)` event loop (returns `Result<(), Box<dyn Error>>`) + private `is_same_position()` and `jiggle_and_scroll()` helpers

**`run(interval: Duration)`** polls mouse position on each tick. If the position is unchanged since the last tick (checked by `is_same_position()`), it calls `jiggle_and_scroll()`, which moves the cursor +1px and back, then sends a zero-delta wheel scroll. A Ctrl+C handler (via `ctrlc`) stops the spinner with a farewell message and terminates the process via `process::exit(0)`.

**Dependencies:** `mouse-rs` (cross-platform mouse control), `spinners` (terminal UX), `ctrlc` (signal handling).

## Platform Notes

- **Linux:** requires `libxdo-dev`; CI runs tests under `xvfb-run` for headless display
- **macOS:** terminal app needs Accessibility permissions
- **Windows:** no extra dependencies

## Lint Strictness

The project enables all five Clippy lint groups (`all`, `pedantic`, `nursery`, `cargo`, `restriction`) plus the rustc lints `missing_docs`, `elided_lifetimes_in_paths`, `bare_trait_objects`, and `explicit_outlives_requirements`, all at `warn`. `[workspace.lints.clippy]` in `Cargo.toml` explicitly allows a set of lints that don't fit this codebase; check that allowlist before suppressing anything locally. Everything else warns, and CI promotes warnings to errors with `-D warnings`.

Key rules:

- Avoid `unwrap()`/`expect()` — propagate errors with `?`. In tests, suppress with `#[expect(clippy::expect_used, reason = "...")]`.
- All public items require doc comments.
- Unsafe blocks require a `// SAFETY:` comment (`undocumented_unsafe_blocks` is active via the restriction group).
- Suppress lints locally with `#[expect(lint, reason = "...")]`, never `#[allow(...)]`.

## CI

`.github/workflows/build.yml`: the Linux job runs fmt, build, clippy (`-D warnings`), `cargo audit` (via `taiki-e/install-action`), `cargo doc` (with `RUSTDOCFLAGS=-D warnings`), and tests under `xvfb-run` (`--locked`); the macOS and Windows jobs only build and test. All jobs use sccache, falling back to uncached builds if it fails to start. A separate `zizmor` job lints the workflows. Dependencies (cargo + GitHub Actions) are kept current via `.github/dependabot.yml` (10-day cooldown).
