# jiggy

[![](https://img.shields.io/github/stars/0xdea/jiggy.svg?style=flat&color=yellow)](https://github.com/0xdea/jiggy)
[![](https://img.shields.io/crates/v/jiggy?style=flat&color=green)](https://crates.io/crates/jiggy)
[![](https://img.shields.io/crates/d/jiggy?style=flat&color=red)](https://crates.io/crates/jiggy)
[![](https://img.shields.io/badge/twitter-%400xdea-blue.svg)](https://twitter.com/0xdea)
[![](https://img.shields.io/badge/mastodon-%40raptor-purple.svg)](https://infosec.exchange/@raptor)
[![build](https://github.com/0xdea/jiggy/actions/workflows/build.yml/badge.svg)](https://github.com/0xdea/jiggy/actions/workflows/build.yml)

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
* As implemented, mouse jiggling won't interfere with your regular activities.
* Thanks to non-invasive mouse wheel scrolling, the new Microsoft Teams should not display you as away.
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

*Note: if run into problems building on Linux, you need to install `libxdo-dev` or equivalent package.*

## Compiling

Alternatively, you can build from [source](https://github.com/0xdea/jiggy):

```sh
$ git clone https://github.com/0xdea/jiggy
$ cd jiggy
$ cargo build --release
```

*Note: if run into problems building on Linux, you need to install `libxdo-dev` or equivalent package.*

## Usage

Run jiggy as follows:

```sh
$ jiggy [check_interval_in_secs]
```

*Note: on macOS, you might need to grant Accessibility privileges to your terminal application.*

## Tested on

* Apple macOS Sequoia 15.2
* Microsoft Windows 11 23H2
* Ubuntu Linux 24.04.1 LTS

## Changelog

* [CHANGELOG.md](CHANGELOG.md)
