#!/usr/bin/env bash

set -x

rustup default nightly-2018-04-12

rustup component add rustfmt-preview

cargo build

cargo rustc -- -Z unstable-options --pretty=expanded -Z trace-macros > expanded-2018-04-12.rs; rustfmt expanded-2018-04-12.rs

rustup default nightly-2018-04-15

rustup component add rustfmt-preview

cargo build

cargo rustc -- -Z unstable-options --pretty=expanded -Z trace-macros > expanded-2018-04-15.rs; rustfmt expanded-2018-04-15.rs
