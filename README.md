# jiggy

[![](https://img.shields.io/github/stars/0xdea/jiggy.svg?style=flat&color=yellow)](https://github.com/0xdea/jiggy)
[![](https://img.shields.io/github/forks/0xdea/jiggy.svg?style=flat&color=green)](https://github.com/0xdea/jiggy)
[![](https://img.shields.io/github/watchers/0xdea/jiggy.svg?style=flat&color=red)](https://github.com/0xdea/jiggy)
[![](https://img.shields.io/crates/v/jiggy?style=flat&color=green)](https://crates.io/crates/jiggy)
[![](https://img.shields.io/crates/d/jiggy?style=flat&color=red)](https://crates.io/crates/jiggy)
[![](https://img.shields.io/badge/twitter-%400xdea-blue.svg)](https://twitter.com/0xdea)
[![](https://img.shields.io/badge/mastodon-%40raptor-purple.svg)](https://infosec.exchange/@raptor)
[![build](https://github.com/0xdea/jiggy/actions/workflows/build.yml/badge.svg)](https://github.com/0xdea/jiggy/actions/workflows/build.yml)
[![doc](https://github.com/0xdea/jiggy/actions/workflows/doc.yml/badge.svg)](https://github.com/0xdea/jiggy/actions/workflows/doc.yml)

> "If I wanted the government in my house I'd buy an Alexa."
>
> -- Rick Sanchez

Jiggy is a minimalistic (but effective) cross-platform mouse jiggler written in Rust. It might be useful for a number of
"reasons".

*Disclaimer: I'm not responsible for any problems that might arise due to using this program to pretend you're "working
from home".*

![](https://raw.githubusercontent.com/0xdea/jiggy/master/.img/working-from-home.jpg)

## Features

* Minimalistic (but effective) mouse jiggler, with no setup needed.
* Mouse jiggling won't interfere with your regular activities.
* Thanks to mouse wheel scroll, the new Microsoft Teams should not display you as away.
* Cross-platform support for macOS, Windows, and Linux.

## See also

* <https://github.com/arkane-systems/mousejiggler>
* <https://lib.rs/crates/jiggle>
* <https://lib.rs/crates/stayawake>
* <https://lib.rs/crates/meth>

## Installing

The easiest way to get the latest release is via [crates.io](https://crates.io/crates/jiggy):

```sh
$ cargo install jiggy
```

TODO - linux (and maybe others) specific lib requirements (libxdo-devel and similar)

## Compiling

Alternatively, you can build from [source](https://github.com/0xdea/jiggy).

On macOS:

```sh
$ git clone https://github.com/0xdea/jiggy
$ cd jiggy
$ cargo build --release
```

On Windows:

```sh
$ git clone https://github.com/0xdea/jiggy
$ cd jiggy
$ cargo build --release

TODO - linux (and maybe others) specific lib requirements (libxdo-devel and similar)
```

On Linux:

```sh
$ git clone https://github.com/0xdea/jiggy
$ cd jiggy
$ cargo build --release

TODO - linux (and maybe others) specific lib requirements (libxdo-devel and similar)
```

## Usage

```sh
jiggy <check_interval_in_secs>
```

## Tested on

* macOS Sequoia 15.2

## Changelog

* [CHANGELOG.md](CHANGELOG.md)
