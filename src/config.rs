//! Contains global config values.
use std::fs::File;
use std::io::{ErrorKind, Read, Write};

const CONFIG_FILE: &str = "user-config.toml";
/// Set config file to `new_value`.
pub fn set_config_file(new_value: &str) {
    let f = File::create(CONFIG_FILE);
    f.unwrap().write_all(new_value.as_bytes()).unwrap();
    println!("Successfully set config at {CONFIG_FILE}.");
}

fn _read_config_file() -> Option<String> {
    let f = File::open(CONFIG_FILE);
    match f {
        Ok(mut f) => {
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            Some(buf)
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => None,
            _ => panic!("Error when trying to open {CONFIG_FILE}: {e}"),
        },
    }
}
