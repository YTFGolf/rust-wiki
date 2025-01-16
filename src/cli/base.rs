use super::cli::ConfigMerge;
use crate::config::config::Config;
use clap::{builder::PossibleValuesParser, Args};
use log::Level;

const POSSIBLE_LOG_LEVELS: [&str; 5] = ["error", "warn", "info", "debug", "trace"];
fn get_levels_values() -> PossibleValuesParser {
    PossibleValuesParser::new(POSSIBLE_LOG_LEVELS)
}

#[derive(Debug, Default, Args, PartialEq)]
/// Options that can apply to every submodule.
pub struct BaseOptions {
    #[arg(value_parser = get_levels_values(), short)]
    /// Log level.
    pub log: Option<Level>,
}
impl ConfigMerge for BaseOptions {
    fn merge(&self, config: &mut Config) {
        if let Some(log) = self.log {
            config.log_level = log;
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
