//! Crate that deals with extracting battle cats info to wikitext. Rewritten in
//! Rust because why not.
#![warn(missing_docs)]
pub mod config;
pub mod data;
mod interface;
pub mod logger;
pub mod meta;
pub mod regex_handler;
pub mod wikitext;

pub use interface::Cli;
