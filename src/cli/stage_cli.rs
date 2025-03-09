//! `stage_info` command.

use super::{
    base::BaseOptions,
    cli_util::{CommandExec, ConfigMerge},
    version_opt::VersionOptions,
};
use crate::{
    cli::cli_util::input, config::Config, data::stage::parsed::stage::Stage,
    meta::stage::stage_types::iter_stage_types, wikitext::stage_info::get_stage_info,
};
use clap::Args;
use std::{
    cmp::max,
    io::{self, Write},
};

fn show_selectors() {
    let mut max_len = 0;
    let mut arrs = vec![];
    for stype in iter_stage_types() {
        let matchers = stype.matcher.arr.join("|");
        max_len = max(max_len, matchers.len());
        arrs.push((matchers, stype.data.name));
    }

    let mut stdout = io::stdout().lock();
    let msg = "Available selectors:";
    writeln!(stdout, "\x1b[4m{msg}\x1b[0m").unwrap();
    // 4 = underline
    for arr in arrs {
        writeln!(
            stdout,
            "{selector:<len$}\t{variant}",
            selector = arr.0,
            len = max_len,
            variant = arr.1
        )
        .unwrap();
    }
    stdout.flush().unwrap();
}

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct StageInfoOptions {
    /// Stage selector.
    pub selector: Vec<String>,

    #[arg(long)]
    /// Do you put `|0` in the Magnification template instead of the actual
    /// magnification for gauntlets?
    pub suppress: Option<bool>,
    #[arg(short, long = "sel")]
    /// Show selector information.
    pub show_sel: bool,

    #[command(flatten)]
    /// Global options.
    pub base: BaseOptions,
    #[command(flatten)]
    /// Version options.
    pub version: VersionOptions,
}
impl ConfigMerge for StageInfoOptions {
    fn merge(&self, config: &mut Config) {
        self.base.merge(config);
        self.version.merge(config);

        let info = &mut config.stage_info;
        if let Some(suppress) = self.suppress {
            info.set_suppress(suppress);
        }
    }
}
impl CommandExec for StageInfoOptions {
    fn exec(&self, config: &Config) {
        if self.show_sel {
            show_selectors();
            return;
        }

        let selector = match self.selector.len() {
            1 => &self.selector[0],
            0 => &input("Input file selector: "),
            _ => &self.selector.join(" "),
        };
        println!(
            "{}",
            get_stage_info(
                &Stage::from_selector(selector, config.version.current_version()).unwrap_or_else(
                    || panic!("Selector {selector:?} doesn't correspond to a real stage.")
                ),
                config
            )
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cli::commands::{Cli, Command},
        config::TEST_CONFIG,
    };
    use clap::Parser;

    #[test]
    fn single_full_selector() {
        const ARGS: [&str; 3] = ["run_program", "stage", "l 0 0"];
        let cli = Cli::parse_from(ARGS.iter());
        assert_eq!(
            cli,
            Cli {
                command: Command::StageInfo(StageInfoOptions {
                    selector: ["l 0 0".to_string()].to_vec(),
                    suppress: Default::default(),
                    base: Default::default(),
                    version: Default::default(),
                    show_sel: Default::default(),
                }),
            }
        );

        let Command::StageInfo(si) = cli.command else {
            unreachable!()
        };
        si.exec(&TEST_CONFIG);
    }

    #[test]
    fn multipart_selector() {
        const ARGS: [&str; 5] = ["run_program", "stage", "l", "0", "0"];
        let cli = Cli::parse_from(ARGS.iter());
        assert_eq!(
            cli,
            Cli {
                command: Command::StageInfo(StageInfoOptions {
                    selector: ["l".to_string(), "0".to_string(), "0".to_string()].to_vec(),
                    suppress: Default::default(),
                    base: Default::default(),
                    version: Default::default(),
                    show_sel: Default::default(),
                }),
            }
        );

        let Command::StageInfo(si) = cli.command else {
            unreachable!()
        };
        si.exec(&TEST_CONFIG);
    }

    #[test]
    fn single_selector() {
        const ARGS: [&str; 3] = ["run_program", "stage", "filibuster"];
        let cli = Cli::parse_from(ARGS.iter());
        assert_eq!(
            cli,
            Cli {
                command: Command::StageInfo(StageInfoOptions {
                    selector: ["filibuster".to_string()].to_vec(),
                    suppress: Default::default(),
                    base: Default::default(),
                    version: Default::default(),
                    show_sel: Default::default(),
                }),
            }
        );

        let Command::StageInfo(si) = cli.command else {
            unreachable!()
        };
        si.exec(&TEST_CONFIG);
    }

    #[test]
    #[should_panic = "Selector \" 0 0\" doesn't correspond to a real stage."]
    fn invalid_selector() {
        const ARGS: [&str; 3] = ["run_program", "stage", " 0 0"];
        let cli = Cli::parse_from(ARGS.iter());
        let Command::StageInfo(si) = cli.command else {
            unreachable!()
        };
        si.exec(&TEST_CONFIG);
    }
}
