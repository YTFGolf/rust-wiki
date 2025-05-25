//! Crate that deals with extracting battle cats info to wikitext. Rewritten in
//! Rust because why not.
#![warn(missing_docs)]
pub mod game_data;
mod interface;
pub mod logger;
pub mod regex_handler;
pub mod wiki_data;
pub mod wikitext;

#[cfg(test)]
pub use interface::TEST_CONFIG;
pub use interface::{CONFIG_FILE, Cli, Config, SLang};
// TODO something about these being fully public
