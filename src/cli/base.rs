//! Config values to use in all cli options.

use super::cli::ConfigMerge;
use crate::{config::Config, logger::set_log_level};
use clap::{
    builder::{PossibleValuesParser, TypedValueParser},
    Args,
};
use log::Level;

const POSSIBLE_LOG_LEVELS: [&str; 5] = ["error", "warn", "info", "debug", "trace"];

#[derive(Debug, Default, Args, PartialEq)]
/// Options that can apply to every submodule.
pub struct BaseOptions {
    #[arg(value_parser = PossibleValuesParser::new(POSSIBLE_LOG_LEVELS).map(|s| s.parse::<Level>().unwrap()))]
    #[arg(ignore_case = true, short)]
    /// Log level.
    pub log: Option<Level>,
}
impl ConfigMerge for BaseOptions {
    fn merge(&self, config: &mut Config) {
        if let Some(log) = self.log {
            config.log_level = log;
            unsafe { set_log_level(log) };
            // I cannot be bothered to uphold safety guarantees so I'll just
            // assume this only will get called once.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;

    #[test]
    fn assert_compile_time_log_levels_is_valid() {
        for (comp, run) in zip(POSSIBLE_LOG_LEVELS, log::Level::iter()) {
            let run = run.as_str().to_lowercase();
            assert_eq!(comp, run);
        }
    }
}
