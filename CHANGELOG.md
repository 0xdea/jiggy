# Changelog for jiggy

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* Enable some lints in the `clippy::restrictions` category.

### Changed

* Optimize release profile options.
* Improve doc comments.
* Update dependencies.

## [1.0.3] - 2026-02-03

### Added

* Introduce the `AUTHORS` constant.

### Changed

* Improve argument parsing.
* Update OS compatibility information.
* Update dependencies.

### Removed

* Remove the `Void` workaround that was there to emulate the never type `!`.

## [1.0.2] - 2026-01-24

### Changed

* Improve command line parsing, error handling, and usage messages.
* Improve code style.
* Improve tests and documentation.
* Update copyright notice.
* Update dependencies.

## [1.0.1] - 2025-12-06

### Changed

* Improve tests.
* Include `README.md` as the crate documentation to avoid writing it twice.
* Update dependencies.

## [1.0.0] - 2025-08-11

### Changed

* Mark as the first stable release.
* Update dependencies.

## [0.1.9] - 2025-06-13

### Changed

* Disable debug info to improve compile time.
* Update dependencies.

## [0.1.8] - 2025-05-23

### Added

* Add contents read permission to build CI.

### Changed

* Improve documentation.
* Update dependencies.

### Fixed

* Update `sccache-action` version.

## [0.1.7] - 2025-03-29

### Changed

* Improve documentation.
* Update dependencies.

## [0.1.6] - 2025-03-20

### Changed

* Improve documentation.
* Update dependencies.

## [0.1.5] - 2025-03-10

### Changed

* Update dependencies.
* Add an error section in public function documentation.
* Avoid generating documentation for private items.
* Improve CI effectiveness and performance.

## [0.1.4] - 2025-03-07

### Changed

* Update dependencies.
* Add `missing_docs` lint and improve documentation.

## [0.1.3] - 2025-02-26

### Changed

* Bump Rust edition to 2024 and update dependencies and CI.
* Improve documentation.
* Improve CI speed by removing redundant tasks.

## [0.1.2] - 2025-02-06

### Fixed

* Get rid of annoying side effects by scrolling the mouse wheel with a zero delta, which is apparently enough to
  keep the computer awake.

## [0.1.1] - 2025-02-04

### Changed

* Improve the welcome and goodbye messages.

## [0.1.0] - 2025-02-03

* First release to be published to [crates.io](https://crates.io/).

[unreleased]: https://github.com/0xdea/jiggy/compare/v1.0.3...HEAD

[1.0.3]: https://github.com/0xdea/jiggy/compare/v1.0.2...v1.0.3

[1.0.2]: https://github.com/0xdea/jiggy/compare/v1.0.1...v1.0.2

[1.0.1]: https://github.com/0xdea/jiggy/compare/v1.0.0...v1.0.1

[1.0.0]: https://github.com/0xdea/jiggy/compare/v0.1.9...v1.0.0

[0.1.9]: https://github.com/0xdea/jiggy/compare/v0.1.8...v0.1.9

[0.1.8]: https://github.com/0xdea/jiggy/compare/v0.1.7...v0.1.8

[0.1.7]: https://github.com/0xdea/jiggy/compare/v0.1.6...v0.1.7

[0.1.6]: https://github.com/0xdea/jiggy/compare/v0.1.5...v0.1.6

[0.1.5]: https://github.com/0xdea/jiggy/compare/v0.1.4...v0.1.5

[0.1.4]: https://github.com/0xdea/jiggy/compare/v0.1.3...v0.1.4

[0.1.3]: https://github.com/0xdea/jiggy/compare/v0.1.2...v0.1.3

[0.1.2]: https://github.com/0xdea/jiggy/compare/v0.1.1...v0.1.2

[0.1.1]: https://github.com/0xdea/jiggy/compare/v0.1.0...v0.1.1

[0.1.0]: https://github.com/0xdea/jiggy/releases/tag/v0.1.0
