use crate::{Config, game_data::cat::parsed::cat::Cat, wikitext::section::Section};

/// Get footer section of page.
pub fn footer(_cat: &Cat, _config: &Config) -> Section {
    // TODO categories, see #
    Section::blank("{{Cats}}")
}
