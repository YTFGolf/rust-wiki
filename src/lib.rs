//! Crate that deals with extracting battle cats info to wikitext. Rewritten in
//! Rust because why not.
#![warn(missing_docs)]
pub mod config;
pub mod game_data;
mod interface;
pub mod logger;
pub mod regex_handler;
pub mod wiki_data;
pub mod wikitext;

pub use interface::Cli;
