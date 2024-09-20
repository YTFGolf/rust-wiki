use crate::config::CONFIG;
use http::header::USER_AGENT;
const WIKI_URL: &str = "https://battlecats.miraheze.org/wiki";

///
pub fn update_wiki_files() -> Result<(), ureq::Error> {
    let test_page = "User:TheWWRNerdGuy/Sandbox";
    let user_agent = format!("{}/rust-wiki-reader", CONFIG.user_name);
    let uri = format!("{WIKI_URL}/{test_page}?action=raw");

    let response = ureq::get(&uri)
        .set(USER_AGENT.as_str(), &user_agent)
        .call()?;

    let content = response.into_string()?;

    println!("{content:?}");
    Ok(())
}
