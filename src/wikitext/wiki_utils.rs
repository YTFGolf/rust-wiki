//! Contains wikitext utilities.

use std::sync::LazyLock;

use regex::Regex;

/// Regexes for utility functions.
pub struct UtilRegexes {
    /// Detects if stage is old or removed.
    pub old_or_removed_detect: Regex,
    /// Captures `[some_char]`. `some_char` is only captured since Rust's
    /// default regex engine doesn't support lookarounds.
    pub old_or_removed_sub: Regex,
}
/// Utility regexes.
pub static REGEXES: LazyLock<UtilRegexes> = LazyLock::new(|| UtilRegexes {
    old_or_removed_detect: Regex::new(r"\((Old|Removed)\)").unwrap(),
    old_or_removed_sub: Regex::new(r" \((?:Old|Removed)\)([^\|\]/]|$)").unwrap(),
});

/**
Extracts the name from a link:

- `[[link|name]]` -> `name`
- `[[link]]` -> `link`
- `name` -> `name`
*/
pub fn extract_name(name: &str) -> &str {
    if name.starts_with("[[") {
        let end = name.find("]]").unwrap();
        match name.find('|') {
            Some(i) => &name[i + 1..end],
            None => &name[2..end],
        }
    } else {
        name
    }
}

// TODO put all wiki files into here
