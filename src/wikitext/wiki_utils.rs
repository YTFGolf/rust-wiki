//! Contains wikitext utilities.

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
            Some(i) => &name[i+1..end],
            None => &name[2..end],
        }
    } else {
        name
    }
}
