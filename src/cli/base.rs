use super::cli2::ConfigMerge;
use crate::config2::config2::Config;
use clap::{
    builder::{PossibleValue, PossibleValuesParser},
    Args,
};
use log::Level;

/// Get possible values parser for log level.
///
/// Do not use this more than necessary, as it leaks memory.
fn get_levels_values() -> PossibleValuesParser {
    let a: Vec<_> = log::Level::iter()
        .map(|v| {
            let b: &str = v.as_str().to_lowercase().leak();
            // Could potentially enable clap's `string` feature so this doesn't
            // need to leak.
            PossibleValue::new(b)
        })
        .collect();
    PossibleValuesParser::new(a)
}

#[derive(Debug, Args, PartialEq)]
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
