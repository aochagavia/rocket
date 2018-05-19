//! A 2D toy game written in Rust, featuring multiple backends.

//#![deny(missing_docs)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![feature(nll)]

extern crate itertools_num;
extern crate rand;

// Note: we need to load `geometry` first so the macro is available for
// the modules that come afterwards
#[macro_use]
pub mod geometry;
pub mod controllers;
pub mod game_state;
pub mod models;
mod util;
