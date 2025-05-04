//! Handler for [`Regex`] idioms.

use regex::Regex;

/// Compile and panic for a statically-defined [`Regex`].
#[track_caller]
pub fn static_regex(pattern: &'static str) -> Regex {
    Regex::new(pattern).expect("static Regex should not fail")
}
