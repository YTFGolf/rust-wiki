//! `read_wiki` command.

use super::util::{CommandExec, ConfigMerge};
use crate::{config::Config, wiki_files::update_wiki_files};
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Encounters options.
pub struct ReadWikiOptions {
    /// Wiki username.
    pub username: Option<String>,
}

impl ConfigMerge for ReadWikiOptions {
    fn merge(&self, config: &mut Config) {
        if let Some(username) = &self.username {
            config.wiki.username = username.clone()
        }
    }
}

impl CommandExec for ReadWikiOptions {
    fn exec(&self, config: &Config) {
        update_wiki_files(config);
    }
}
