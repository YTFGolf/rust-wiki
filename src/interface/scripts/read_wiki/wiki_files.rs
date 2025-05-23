//! Module that deals with getting and updating wiki files.

use crate::{
    config::Config,
    wikitext::file_handler::{FileLocation, get_file_location},
};
use similar::{ChangeTag, TextDiff};
use std::{
    fs::File,
    io::{Read, Write},
};
const WIKI_URL: &str = "https://battlecats.miraheze.org/wiki";

/// (`file_name`, `page_name`)
const FILES: [(&str, &str); 7] = [
    ("StageNames.csv", "User:TheWWRNerdGuy/data/StageNames.csv"),
    (
        "EnemyLinkData.csv",
        "User:TheWWRNerdGuy/data/EnemyLinkData.csv",
    ),
    ("Treasures.csv", "User:TheWWRNerdGuy/data/Treasures.csv"),
    ("Difficulty.txt", "User:Novastrala/Difficulty.txt"),
    ("UnitNames.csv", "Module:Cats/names.csv"),
    ("EnemyNames.csv", "Module:Enemies.csv"),
    (
        "ContinueStages.csv",
        "User:TheWWRNerdGuy/data/ContinueStages.csv",
    ),
];

const USER_AGENT: &str = "user-agent";

/// Get a coloured unified diff between the old and new content.
fn get_file_diff(old_content: &str, new_content: &str) -> String {
    // This is largely copied from the implementation of `similar`'s unified
    // diff's format trait.
    let binding = TextDiff::configure().diff_lines(old_content, new_content);
    let diff = binding.unified_diff();

    let mut buf = Vec::new();

    for hunk in diff.iter_hunks() {
        for (idx, change) in hunk.iter_changes().enumerate() {
            if idx == 0 {
                writeln!(buf, "{}", hunk.header()).unwrap();
            }
            match change.tag() {
                ChangeTag::Insert => write!(buf, "\x1b[38;2;0;200;0m").unwrap(),
                ChangeTag::Delete => write!(buf, "\x1b[38;2;200;0;0m").unwrap(),
                ChangeTag::Equal => (),
            };
            write!(buf, "{}{}", change.tag(), change.to_string_lossy()).unwrap();
            write!(buf, "\x1b[38;2;255;255;255m").unwrap();
        }
    }
    let diff: String = String::from_utf8(buf).unwrap();
    diff
}

/// Get rid of the `<pre>` and `</pre>` parts of the page's content.
fn strip_pre(content: &str) -> &str {
    const PRE_START: &str = "<pre>\n";
    const PRE_END: &str = "</pre>\n";
    if content.starts_with(PRE_START) {
        &content[PRE_START.len()..content.len() - PRE_END.len()]
    } else {
        content
    }
}

/// Goes through all files stored on teh wiki and updates the local versions of
/// each.
pub fn update_wiki_files(config: &Config) {
    let directory = get_file_location(&FileLocation::WikiData);
    std::fs::create_dir_all(directory).unwrap();

    let user_agent = format!("{}/rust-wiki-reader", config.wiki.username);
    for (file_name, page_name) in FILES {
        let uri = format!("{WIKI_URL}/{page_name}?action=raw");
        let response = ureq::get(&uri)
            .header(USER_AGENT, &user_agent)
            .call()
            .expect("Error: couldn't get the data from the wiki.");
        let mut res_str = response.into_body().read_to_string().unwrap();
        res_str.push('\n');
        let content = strip_pre(&res_str);

        let path = directory.join(file_name);
        let file = File::options().read(true).open(&path);

        let diff: String = match file {
            Ok(mut f) => {
                let mut buf = String::new();
                f.read_to_string(&mut buf).unwrap();
                get_file_diff(&buf, content)
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => "File created".to_string(),
                _ => panic!("Error when trying to open file: {e}"),
            },
        };

        if diff.is_empty() {
            continue;
        }

        println!("{:#>80}\n{file_name}", '#');
        println!("{diff}\n");
        let mut f_write = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        f_write.write_all(content.as_bytes()).unwrap();
    }
}
