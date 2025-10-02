//! Deals with talents section.

use crate::{
    game_data::cat::parsed::cat::Cat, interface::config::Config, wikitext::section::Section,
};

/// Get the talents section.
pub fn talents_section(cat: &Cat, config: &Config) -> Option<Section> {
    let talents = cat.get_talents(config.version.current_version())?;

    println!("{talents:#?}");
    // for talent in talents.groups.iter() {
    //     println!("{talent:?}");
    //     println!(
    //         "abilityID_X = {}",
    //         TALENT_DATA.get_talent_name(talent.abilityID_X.into())
    //     )
    // }

    if true {
        panic!();
    }

    None
}
