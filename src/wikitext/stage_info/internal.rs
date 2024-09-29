//! Internal functions for stage info.

// TODO document these
mod beginning;
mod enemies_list;
mod information;
mod treasure;
pub mod test_util;
use crate::{data::stage::parsed::stage::Stage, wikitext::template_parameter::TemplateParameter};
pub use beginning::{enemies_appearing, intro};
pub use enemies_list::enemies_list;
pub use information::{base_hp, energy, stage_location, stage_name};
pub use treasure::{score_rewards, treasure};
use std::io::Write;

pub fn restrictions_section(_stage: &Stage) -> Vec<u8> {
    vec![]
}

pub fn param_vec_fold(mut buf: Vec<u8>, param: TemplateParameter) -> Vec<u8> {
    let smallbuf = param.to_u8s();
    buf.extend(smallbuf.iter());
    buf.write(b"\n").unwrap();
    buf
}

// Should probably also test param_vec_fold
/*
        let rong_buf = base_hp(&rongorongo)
            .into_iter()
            .fold(vec![], param_vec_fold);
        assert_eq!(
            &String::from_utf8(rong_buf).unwrap(),
            "\
        |enemy castle hp = 300,000 HP\n\
        |enemy castle hp2 = 450,000 HP\n\
        |enemy castle hp3 = 600,000 HP\n\
        |enemy castle hp4 = 900,000 HP\n\
        "
        );
        // FIXME the end here shouldn't have a "\n" but it makes no difference
        // when doing the format so I CBA to fix it rn
*/
