//! Get info about a cat.

use crate::config::Config;

/// Do thing.
pub fn do_thing(wiki_id: u32, config: &Config) {
    println!("{wiki_id:?} {config:?}")
}
