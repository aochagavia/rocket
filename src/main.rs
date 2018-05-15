//! A 2D toy game written in Rust, using the ggez library.
#![deny(missing_docs)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![feature(nll)]

extern crate ggez;
extern crate itertools_num;
extern crate rand;

// Note: we need to load `geometry` first so the macro is available for
// the modules that come afterwards
#[macro_use]
mod geometry;
mod controllers;
mod game_state;
mod models;
mod platform;
mod util;
mod view;

fn main() {
    // The main function will be chosen depending on the target (wasm or ggez)
    platform::main();
}
