//! Internal functions for stage info.

// TODO document these
mod beginning;
mod enemies_list;
mod information;
mod restrictions;
pub mod test_util;
mod treasure;
pub use beginning::{enemies_appearing, intro};
pub use enemies_list::enemies_list;
pub use information::{base_hp, energy, stage_location, stage_name};
pub use restrictions::{restrictions_info, restrictions_section};
pub use treasure::{score_rewards, treasure};
