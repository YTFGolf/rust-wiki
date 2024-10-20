//! Contains wikitext utilities.

use regex::Regex;
use std::sync::LazyLock;

/// Regular expressions for wikitext functions.
pub struct UtilRegexes {
    /// Detects if stage is old or removed.
    pub old_or_removed_detect: Regex,
    /// Used for getting rid of the ` (Old)` or ` (Removed)` at the end of stage
    /// or map names.
    ///
    /// Captures `[some_char]` (i.e. you'll need to replace with `"$1"` instead
    /// of standard replacing with `""`). `some_char` is only captured since
    /// Regex crate doesn't support lookarounds.
    ///
    /// ```
    /// # use rust_wiki::wikitext::wiki_utils::REGEXES;
    /// let map_name = "[[Deleted Event]] (Removed)";
    /// assert_eq!(REGEXES.old_or_removed_sub.replace_all(&map_name, "$1"), "[[Deleted Event]]");
    /// ```
    pub old_or_removed_sub: Regex,
}
/// Utility regexes.
pub static REGEXES: LazyLock<UtilRegexes> = LazyLock::new(|| UtilRegexes {
    old_or_removed_detect: Regex::new(r"\((Old|Removed)\)").unwrap(),
    old_or_removed_sub: Regex::new(r" \((?:Old|Removed)\)([^\|\]/]|$)").unwrap(),
});

/**
Extracts the name from a link:

```
# use rust_wiki::wikitext::wiki_utils::extract_name;
assert_eq!(extract_name("[[link|name]]"), "name");
assert_eq!(extract_name("[[link]]"), "link");
assert_eq!(extract_name("name"), "name");
```
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

// TODO put container for wiki file data into here instead of `data_files`
