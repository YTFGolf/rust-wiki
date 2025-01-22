//! `read_wiki` command.

use super::{
    base::BaseOptions,
    cli_util::{CommandExec, ConfigMerge},
};
use crate::{config::Config, wiki_files::update_wiki_files};
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Encounters options.
pub struct ReadWikiOptions {
    /// Wiki username.
    pub username: Option<String>,

    #[command(flatten)]
    /// Global options.
    pub base: BaseOptions,
}

impl ConfigMerge for ReadWikiOptions {
    fn merge(&self, config: &mut Config) {
        self.base.merge(config);
        if let Some(username) = &self.username {
            config.wiki.username.clone_from(username);
        }
    }
}

impl CommandExec for ReadWikiOptions {
    fn exec(&self, config: &Config) {
        update_wiki_files(config);
    }
}
