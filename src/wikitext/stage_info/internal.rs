//! Internal functions for stage info.

// TODO document these
mod battlegrounds;
mod beginning;
mod enemies_list;
mod information;
mod misc_information;
mod restrictions;
pub mod test_util;
mod treasure;
use crate::data::stage::parsed::stage::Stage;
pub use battlegrounds::battlegrounds;
pub use beginning::{enemies_appearing, intro};
pub use enemies_list::enemies_list;
pub use information::{base_hp, energy, max_enemies, stage_location, stage_name, width, xp};
pub use misc_information::{chapter, difficulty, max_clears, stage_nav, star};
pub use restrictions::{restrictions_info, restrictions_section};
pub use treasure::{score_rewards, treasure};

pub fn reference(stage: &Stage) -> String {
    format!(
        "https://battlecats-db.com/stage/s{type:02}{map:03}-{incremented_stage:02}.html",
        r#type = stage.meta.type_num,
        map = stage.meta.map_num,
        incremented_stage = stage.meta.stage_num + 1,
    )
}
