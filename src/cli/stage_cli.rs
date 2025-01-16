//     Command::StageInfo(si) => {
//         let config = &get_config(config, si.config.clone());
//         stage_info(si, config);
//     }

use super::base::BaseOptions;
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct StageInfoOptions {
    /// Stage selector.
    pub selector: Vec<String>,

    #[command(flatten)]
    pub base: BaseOptions,
}
