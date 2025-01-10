//! Contains wikitext utilities.

use regex::Regex;
use std::sync::LazyLock;

// Utility regexes.
/// Detects if stage is old or removed.
pub static OLD_OR_REMOVED_DETECT: LazyLock<Regex> = LazyLock::new(|| Regex::new(DET_PAT).unwrap());
const DET_PAT: &str = r"\((Old|Removed)\)";

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
pub static OLD_OR_REMOVED_SUB: LazyLock<Regex> = LazyLock::new(|| Regex::new(SUB_PAT).unwrap());
const SUB_PAT: &str = r" \((?:Old|Removed)\)([^\|\]/]|$)";

/**
Extracts the name from a link:

```
# use rust_wiki::wikitext::wiki_utils::extract_name;
assert_eq!(extract_name("[[link|name]]"), "name");
assert_eq!(extract_name("[[link]]"),      "link");
assert_eq!(extract_name("name"),          "name");

const COTC: &str = "[[Cats of the Cosmos]] [[Zombie Outbreaks|Outbreaks]] 2";
assert_eq!(extract_name(COTC),            COTC);
```
*/
pub fn extract_name(name: &str) -> &str {
    if name.starts_with("[[") && name.chars().filter(|c| *c == '[').count() <= 2 {
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
