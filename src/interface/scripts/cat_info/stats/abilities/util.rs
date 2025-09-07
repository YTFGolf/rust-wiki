//! Utility functions for abilities section.

use crate::{
    interface::error_handler::InfallibleWrite,
    wikitext::number_utils::{plural_f, time_repr},
};
use num_format::{Locale, WriteFormatted};
use std::fmt::Write;

/// Get string representation of duration (with `<sub>` tags).
pub fn get_duration_repr(duration: u32) -> String {
    let (dur_f, dur_s) = time_repr(duration);
    format!(
        "{dur_f}f <sub>{dur_s} {seconds}</sub>",
        seconds = plural_f(duration, "second", "seconds")
    )
}

/// Get link to special abilities page.
pub fn get_ability(link: &str, display: &str) -> String {
    format!("[[Special Abilities#{link}|{display}]]")
}

/// Get link to abilities page where link == display.
pub fn get_ability_single(link_display: &str) -> String {
    format!("[[Special Abilities#{link_display}|{link_display}]]")
}

/// Get link to enemy category.
pub fn get_enemy_category(link: &str, display: &str) -> String {
    format!("[[:Category:{link} Enemies|{display}]]")
}

/// Get representation of ld/omnistrike range.
pub fn get_range_repr(min: i16, max: i16) -> String {
    let mut buf = String::new();
    buf.write_formatted(&min, &Locale::en).infallible_write();
    if min == max {
        return buf;
    }

    buf.write_char('~').infallible_write();
    buf.write_formatted(&max, &Locale::en).infallible_write();

    buf
}
