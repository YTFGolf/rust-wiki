//! Crate that deals with extracting battle cats info to wikitext. Rewritten in
//! Rust because why not.
#![warn(missing_docs)]
pub mod config;
pub mod data;
pub mod file_handler;
pub mod wiki_files;
pub mod cli;
pub mod wikitext;
