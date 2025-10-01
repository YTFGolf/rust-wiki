//! Parsed talents object.

#[cfg(test)]
mod tests {
    use crate::{
        TEST_CONFIG, game_data::cat::raw::talents::TalentsContainer,
        wiki_data::talent_names::TALENT_DATA,
    };

    use super::*;

    #[test]
    fn check_all_talents() {
        let version = TEST_CONFIG.version.current_version();
        let talents_cont = version.get_cached_file::<TalentsContainer>();
        for talents in talents_cont.iter() {
            println!("{:?}", talents.fixed);
            for talent in talents.groups.iter() {
                if talent.abilityID_X == 0 {
                    continue;
                }

                println!("{talent:?}");
                println!(
                    "abilityID_X = {:?}",
                    TALENT_DATA.get_talent_name(talent.abilityID_X.into())
                )
            }

            // println!("{talents:?}");
            println!("");
        }
        todo!()
    }
}
