//! Module that deals with getting and updating wiki files.

use crate::{
    config::CONFIG,
    file_handler::{get_file_location, FileLocation},
};
use http::header::USER_AGENT;
use std::{fs::File, io::Read};
const WIKI_URL: &str = "https://battlecats.miraheze.org/wiki";

/// (file name, wiki page name)
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

fn get_file_diff(old_content: String, new_content: String) -> String {
    // let a = text_diff::diff(&old_content, &new_content, "\n");
    // text_diff::print_diff(&old_content, &new_content, "\n");
    // for b in a.1.iter(){
    //     println!("{:?}", b.)
    // }
    // println!("{}", );
    todo!()
}

///
pub fn update_wiki_files() -> Result<(), ureq::Error> {
    let directory = get_file_location(FileLocation::WikiData);
    std::fs::create_dir_all(directory).unwrap();

    let user_agent = format!("{}/rust-wiki-reader", CONFIG.user_name);
    for (file_name, page_name) in FILES {
        let uri = format!("{WIKI_URL}/{page_name}?action=raw");
        let response = ureq::get(&uri)
            .set(USER_AGENT.as_str(), &user_agent)
            .call()?;
        let content = response.into_string()?;

        let path = directory.join(file_name);
        let file = File::options().read(true).open(&path);

        let diff: String = match file {
            Ok(mut f) => {
                let mut buf = String::new();
                f.read_to_string(&mut buf).unwrap();
                get_file_diff(buf, content)
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => "File created".to_string(),
                _ => panic!("Error when trying to open file: {e}"),
            },
        };

        println!("{:#>80}\n{file_name}", '#');
        println!("{diff}");
        let x = File::options().write(true).create(true).open(&path);
        panic!("{x:?}")
    }

    Ok(())
}

// let mut out = File::create("test").expect("failed to create file");
// io::copy(&mut resp, &mut out).expect("failed to copy content");
