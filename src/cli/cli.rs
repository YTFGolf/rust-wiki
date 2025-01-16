//! Represents the command line interface.

use crate::config2::config2::Config;

/// Overwrite values of a [`Config`] object.
pub trait ConfigMerge {
    /// Overwrites values of `config` with equivalent values from `&self`.
    fn merge(&self, config: &mut Config);

    // /// Returnable implementation of [ConfigMerge::merge].
    // fn combine(&self, mut config: Config) -> Config {
    //     self.merge(&mut config);
    //     config
    // }
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
