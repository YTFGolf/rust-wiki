//! Crate that deals with extracting battle cats info to wikitext. Rewritten in
//! Rust because why not.
#![warn(missing_docs)]
pub mod game_data;
mod interface;
pub mod logger;
pub mod regex_handler;
pub mod wiki_data;
pub mod wikitext;

pub use interface::CONFIG_FILE;
pub use interface::Cli;
pub use interface::Config;
pub use interface::SLang;
#[cfg(test)]
pub use interface::TEST_CONFIG;
