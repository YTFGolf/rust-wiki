//! Module that deals with getting and updating wiki files.

use crate::{
    config::CONFIG,
    file_handler::{get_file_location, FileLocation},
};
use http::header::USER_AGENT;
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
                _ => (),
            };
            write!(buf, "{}{}", change.tag(), change.to_string_lossy()).unwrap();
            write!(buf, "\x1b[38;2;255;255;255m").unwrap();
            // FIXME doesn't show newline at end of file, just shows that the
            // line is changed but without any explanation.
        }
    }
    let a: String = String::from_utf8(buf).unwrap();
    a
}

/// Get rid of the `<pre>` and `</pre>` parts of the page's content.
fn strip_pre(content: &str) -> &str {
    if content.starts_with("<pre>\n") {
        &content["<pre>\n".len()..content.len() - "</pre>".len()]
    } else {
        content
    }
}

/// Goes through all files stored on teh wiki and updates the local versions of
/// each.
pub fn update_wiki_files() -> Result<(), ureq::Error> {
    let directory = get_file_location(FileLocation::WikiData);
    std::fs::create_dir_all(directory).unwrap();

    let user_agent = format!("{}/rust-wiki-reader", CONFIG.user_name);
    for (file_name, page_name) in FILES {
        let uri = format!("{WIKI_URL}/{page_name}?action=raw");
        let response = ureq::get(&uri)
            .set(USER_AGENT.as_str(), &user_agent)
            .call()?;
        let res_str = response.into_string()?;
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
            .open(&path)
            .unwrap();
        f_write.write_all(content.as_bytes()).unwrap();
    }

    Ok(())
}
