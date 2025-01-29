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
/// # use rust_wiki::wikitext::wiki_utils::OLD_OR_REMOVED_SUB;
/// let map_name = "[[Deleted Event]] (Removed)";
/// assert_eq!(OLD_OR_REMOVED_SUB.replace_all(&map_name, "$1"), "[[Deleted Event]]");
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

/**
Extracts the link from a formatted link:

```
# use rust_wiki::wikitext::wiki_utils::extract_link;
assert_eq!(extract_link("[[link|name]]"), "link");
assert_eq!(extract_link("[[link]]"),      "link");
assert_eq!(extract_link("nothing"),       "nothing");

const COTC: &str = "[[Cats of the Cosmos]] [[Zombie Outbreaks|Outbreaks]] 2";
assert_eq!(extract_link(COTC),            COTC);
```
*/
pub fn extract_link(link: &str) -> &str {
    if link.starts_with("[[") && link.chars().filter(|c| *c == '[').count() <= 2 {
        let end = link.find("]]").unwrap();
        match link.find('|') {
            Some(i) => &link[2..i],
            None => &link[2..end],
        }
    } else {
        link
    }
}

/// Get the ordinal number corresponding to `n` (e.g. 1 = "first").
pub fn get_ordinal(n: u32) -> String {
    assert!((1..1000).contains(&n));

    const SMALL_ORDS: [&str; 9] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
    ];

    if n as usize <= SMALL_ORDS.len() {
        return SMALL_ORDS[n as usize - 1].to_string();
    }

    let m = n % 100;
    if (11..=13).contains(&m) {
        format!("{n}th")
    } else if m % 10 == 1 {
        format!("{n}st")
    } else if m % 10 == 2 {
        format!("{n}nd")
    } else if m % 10 == 3 {
        format!("{n}rd")
    } else {
        format!("{n}th")
    }
}

#[cfg(test)]
mod tests {
    use crate::wikitext::wiki_utils::get_ordinal;

    #[test]
    #[should_panic = "assertion failed: (1..1000).contains(&n)"]
    fn ordinal_0() {
        let n = 0;
        get_ordinal(n);
    }

    #[test]
    fn ordinal_small() {
        let n = 4;
        assert_eq!(get_ordinal(n), "fourth")
    }

    #[test]
    fn ordinal_12() {
        let n = 12;
        assert_eq!(get_ordinal(n), "12th")
    }

    #[test]
    fn ordinal_large() {
        let n = 42;
        assert_eq!(get_ordinal(n), "42nd")
    }

    #[test]
    fn ordinal_large_small() {
        let n = 91;
        assert_eq!(get_ordinal(n), "91st")
    }

    #[test]
    fn ordinal_100() {
        let n = 100;
        assert_eq!(get_ordinal(n), "100th")
    }

    #[test]
    fn ordinal_113() {
        let n = 113;
        assert_eq!(get_ordinal(n), "113th")
    }
}
