#![warn(unused_crate_dependencies)]
#![forbid(unsafe_code)]
#![warn(clippy::all)]

//! This crate provides a parser for the Action Language that is used to generate the stories in
//! the game.

pub mod parser;

mod string;

pub use parser::{actions, Action};
