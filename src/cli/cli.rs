//! Represents the command line interface.

use crate::config::Config;
use std::io::{self, Write};

/// Syntax sugar for a function that works like Python's `input`.
pub fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin().lines().next().unwrap().unwrap()
}

/// Overwrite values of a [`Config`] object.
pub trait ConfigMerge {
    /// Overwrite values of `config` with equivalent values from `&self`.
    fn merge(&self, config: &mut Config);
}

/// Execute a CLI command.
pub trait CommandExec {
    /// Execute the command.
    fn exec(&self, config: &Config);
}

/// Run the CLI command with given options and config.
pub trait CliCommand: ConfigMerge + CommandExec {
    ///
    fn run(&self, mut config: Config) {
        self.merge(&mut config);
        self.exec(&config);
    }
}

impl<T> CliCommand for T where T: ConfigMerge + CommandExec {}
