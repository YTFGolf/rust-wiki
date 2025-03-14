//! Deals with basic stage information in the infobox.

use crate::{
    data::stage::parsed::stage::Stage,
    meta::stage::{stage_types::transform::transform_map::map_img_code, variant::StageVariantID},
    wikitext::{
        data_files::{enemy_data::ENEMY_DATA, rewards::TREASURE_DATA},
        template_parameter::TemplateParameter,
    },
};
use either::Either::{Left, Right};
use num_format::{Locale, WriteFormatted};
use std::fmt::Write;

/// Get the `|stage name` parameter.
pub fn stage_name(stage: &Stage) -> TemplateParameter {
    let mut buf = String::new();

    match stage.anim_base_id {
        None => write!(buf, "[[File:rc{base_id:03}.png]]", base_id = stage.base_id).unwrap(),
        Some(id) => {
            let id: u32 = u32::from(id) - 2;
            const RESIZE: [u32; 5] = [657, 669, 678, 681, 693];
            if RESIZE.contains(&id) {
                write!(buf, "[[File:E {id}.png|250px]]").unwrap();
            } else {
                write!(buf, "[[File:E {id}.png]]").unwrap();
                // maybe just put the 250px there always
            }
        }
    };
    // base part

    write!(
        buf,
        "\n[[File:Mapsn{map_num:03} {stage_num:02} {type_code} en.png]]",
        map_num = stage.id.map().num(),
        stage_num = stage.id.num(),
        type_code = map_img_code(stage.id.map()),
    )
    .unwrap();
    // stage name part

    TemplateParameter::new("stage name", buf)
}

/// Get the `|stage location` parameter.
pub fn stage_location(stage: &Stage) -> TemplateParameter {
    let buf = format!(
        "[[File:Mapname{map_num:03} {type_code} en.png]]",
        map_num = stage.id.map().num(),
        type_code = map_img_code(stage.id.map())
    );
    TemplateParameter::new("stage location", buf)
}

fn energy_catamin(cost: u32) -> TemplateParameter {
    const CATAMIN_A_ID: u32 = 55;

    // e.g. 2002 for Catamin C x2
    let catamin_type = cost / 1000;
    let amount = cost % 1000;
    let name = TREASURE_DATA.get_treasure_name(CATAMIN_A_ID + catamin_type);

    TemplateParameter::new("catamins", format!("{name} x{amount}"))
}

/// Get the `|energy` parameter.
pub fn energy(stage: &Stage) -> Option<TemplateParameter> {
    let energy = stage.energy?;
    let amount = match stage.id.variant() {
        StageVariantID::Catamin => return Some(energy_catamin(energy)),
        StageVariantID::Extra => "N/A".to_string(),
        _ => {
            let mut buf = String::new();
            buf.write_formatted(&energy, &Locale::en).unwrap();
            buf
        }
    };

    Some(TemplateParameter::new("energy", amount))
}

/// Get the `|enemy castle hp` parameters.
pub fn base_hp(stage: &Stage) -> Vec<TemplateParameter> {
    const PARAM_NAME: &str = "enemy castle hp";
    const PARAM_NAME_2: &str = "enemy castle hp2";
    const PARAM_NAME_3: &str = "enemy castle hp3";
    const PARAM_NAME_4: &str = "enemy castle hp4";

    if stage.time_limit.is_some() {
        return vec![TemplateParameter::new(PARAM_NAME, "Unlimited".to_string())];
    }
    // Dojo
    if stage.anim_base_id.is_none() {
        let mut buf = String::new();
        buf.write_formatted(&stage.base_hp, &Locale::en).unwrap();
        buf.write_str(" HP").unwrap();
        return vec![TemplateParameter::new(PARAM_NAME, buf)];
    }

    let anim_base_id = <u32>::from(stage.anim_base_id.unwrap()) - 2;
    let base_hp = ENEMY_DATA.get_data(anim_base_id).hp;
    let enemy_magnification = || {
        for enemy in &stage.enemies {
            if enemy.is_base {
                return enemy.magnification;
            }
            // won't always be first enemy in stage e.g. clown bases so need to
            // check all
        }
        unreachable!()
    };
    let mag = match enemy_magnification() {
        Left(m) => m,
        Right((hp, _ap)) => hp,
    };

    let magnification_hp = mag * base_hp / 100;
    if stage.crown_data.is_none() {
        let mut buf = String::new();
        buf.write_formatted(&magnification_hp, &Locale::en).unwrap();
        buf.write_str(" HP").unwrap();
        return vec![TemplateParameter::new(PARAM_NAME, buf)];
    }

    let mut params = vec![];
    let get_new_param = |key, value| {
        let mut buf = String::new();
        buf.write_formatted(&value, &Locale::en).unwrap();
        buf.write_str(" HP").unwrap();
        TemplateParameter::new(key, buf)
    };

    let crown_data = stage.crown_data.as_ref().unwrap();
    params.push(get_new_param(PARAM_NAME, magnification_hp));

    if let Some(m) = crown_data.crown_2 {
        params.push(get_new_param(
            PARAM_NAME_2,
            magnification_hp * u32::from(m) / 100,
        ));
    }

    if let Some(m) = crown_data.crown_3 {
        params.push(get_new_param(
            PARAM_NAME_3,
            magnification_hp * u32::from(m) / 100,
        ));
    }

    if let Some(m) = crown_data.crown_4 {
        if u32::from(m) != 100 {
            params.push(get_new_param(
                PARAM_NAME_4,
                magnification_hp * u32::from(m) / 100,
            ));
        }
        // wiki templates don't need 4-crown mags if they are the same as
        // 1-crown
    }

    params
}

/// Get the xp drop of a stage.
pub fn xp(stage: &Stage) -> Option<TemplateParameter> {
    let xp = stage.xp?;
    if stage.id.variant() == StageVariantID::RankingDojo && xp == 0 {
        return None;
    }
    let mut buf = String::new();
    buf.write_formatted(&xp, &Locale::en).unwrap();
    buf.write_str(" XP").unwrap();

    Some(TemplateParameter::new("XP", buf))
}

/// Get the width of a stage.
pub fn width(stage: &Stage) -> TemplateParameter {
    let mut buf = String::new();
    buf.write_formatted(&stage.width, &Locale::en).unwrap();

    TemplateParameter::new("width", buf)
}

/// Get the max enemies of a stage.
pub fn max_enemies(stage: &Stage) -> TemplateParameter {
    let mut buf = String::new();
    buf.write_formatted(&stage.max_enemies, &Locale::en)
        .unwrap();

    TemplateParameter::new("max enemies", buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::stage::stage_id::StageID;
    use StageVariantID as T;

    #[test]
    fn test_stage_name_and_loc() {
        let great_escaper =
            Stage::from_id_current(StageID::from_components(T::SoL, 17, 5)).unwrap();
        let mut buf = String::new();
        buf.write_str(&stage_name(&great_escaper).to_string())
            .unwrap();
        buf.write_str("\n").unwrap();
        buf.write_str(&stage_location(&great_escaper).to_string())
            .unwrap();
        assert_eq!(
            buf,
            "\
            |stage name = [[File:rc006.png]]\n\
            [[File:Mapsn017 05 n en.png]]\n\
            |stage location = [[File:Mapname017 n en.png]]\
            "
        );

        let red_summit = Stage::new_current("h 10 0").unwrap();
        let mut buf = String::new();
        buf.write_str(&stage_name(&red_summit).to_string()).unwrap();
        buf.write_str("\n").unwrap();
        buf.write_str(&stage_location(&red_summit).to_string())
            .unwrap();
        assert_eq!(
            buf,
            "\
            |stage name = [[File:rc002.png]]\n\
            [[File:Mapsn010 00 h en.png]]\n\
            |stage location = [[File:Mapname010 h en.png]]\
            "
        );

        let finale = Stage::from_id_current(StageID::from_components(T::Collab, 209, 0)).unwrap();
        let mut buf = String::new();
        buf.write_str(&stage_name(&finale).to_string()).unwrap();
        buf.write_str("\n").unwrap();
        buf.write_str(&stage_location(&finale).to_string()).unwrap();
        assert_eq!(
            buf,
            "\
            |stage name = [[File:E 651.png]]\n\
            [[File:Mapsn209 00 c en.png]]\n\
            |stage location = [[File:Mapname209 c en.png]]\
            "
        );

        let relay_1600m =
            Stage::from_id_current(StageID::from_components(T::Extra, 61, 2)).unwrap();
        let mut buf = String::new();
        buf.write_str(&stage_name(&relay_1600m).to_string())
            .unwrap();
        assert_eq!(
            buf,
            "\
            |stage name = [[File:E 657.png|250px]]\n\
            [[File:Mapsn061 02 ex en.png]]\
            "
        );
    }

    #[test]
    fn test_energy_normal() {
        let aac = Stage::from_id_current(StageID::from_components(T::UL, 0, 0)).unwrap();
        assert_eq!(
            energy(&aac),
            Some(TemplateParameter::new("energy", "200".to_string()))
        );
    }

    #[test]
    fn test_energy_0() {
        let challenge =
            Stage::from_id_current(StageID::from_components(T::Challenge, 0, 0)).unwrap();
        assert_eq!(
            energy(&challenge),
            Some(TemplateParameter::new("energy", "0".to_string()))
        );
    }

    #[test]
    fn test_energy_ex() {
        let door_opens = Stage::from_id_current(StageID::from_components(T::Extra, 47, 0)).unwrap();
        assert_eq!(
            energy(&door_opens),
            Some(TemplateParameter::new("energy", "N/A".to_string()))
        );
    }

    #[test]
    fn test_energy_catamin() {
        let facing_danger =
            Stage::from_id_current(StageID::from_components(T::Catamin, 5, 0)).unwrap();
        assert_eq!(
            energy(&facing_danger),
            Some(TemplateParameter::new(
                "catamins",
                "[[Catamin]] [C] x2".to_string()
            ))
        );
    }

    #[test]
    fn test_energy_1_000() {
        let mining_epic =
            Stage::from_id_current(StageID::from_components(T::Event, 326, 0)).unwrap();
        assert_eq!(
            energy(&mining_epic),
            Some(TemplateParameter::new("energy", "1,000".to_string()))
        );
    }

    #[test]
    fn test_energy_labyrinth() {
        let labyrinth_67 =
            Stage::from_id_current(StageID::from_components(T::Labyrinth, 0, 66)).unwrap();
        assert_eq!(energy(&labyrinth_67), None);
    }

    #[test]
    fn test_base_hp_normal() {
        let ht30 = Stage::from_id_current(StageID::from_components(T::Tower, 0, 29)).unwrap();
        assert_eq!(
            base_hp(&ht30),
            vec![TemplateParameter::new(
                "enemy castle hp",
                "1,000,000 HP".to_string()
            )]
        );
    }

    #[test]
    fn test_base_hp_dojo() {
        let dojo = Stage::from_id_current(StageID::from_components(T::Dojo, 0, 0)).unwrap();
        assert_eq!(
            base_hp(&dojo),
            vec![TemplateParameter::new(
                "enemy castle hp",
                "Unlimited".to_string()
            )]
        );
    }

    #[test]
    fn test_base_hp_mismatch() {
        // where stage.base_hp != actual base hp
        let just_friends =
            Stage::from_id_current(StageID::from_components(T::Event, 302, 2)).unwrap();
        assert_eq!(just_friends.base_hp, 10);
        assert_eq!(
            base_hp(&just_friends),
            vec![TemplateParameter::new(
                "enemy castle hp",
                "30,000 HP".to_string()
            )]
        );

        let finale = Stage::from_id_current(StageID::from_components(T::Collab, 209, 0)).unwrap();
        assert_eq!(finale.base_hp, 1_000);
        assert_eq!(
            base_hp(&finale),
            vec![TemplateParameter::new(
                "enemy castle hp",
                "50 HP".to_string()
            )]
        );
    }

    #[test]
    fn test_base_hp_starred() {
        let rongorongo =
            Stage::from_id_current(StageID::from_components(T::Event, 129, 5)).unwrap();
        assert_eq!(rongorongo.base_hp, 300_000);
        assert_eq!(
            base_hp(&rongorongo),
            vec![
                TemplateParameter::new("enemy castle hp", "300,000 HP".to_string()),
                TemplateParameter::new("enemy castle hp2", "450,000 HP".to_string()),
                TemplateParameter::new("enemy castle hp3", "600,000 HP".to_string()),
                TemplateParameter::new("enemy castle hp4", "900,000 HP".to_string()),
            ]
        );
    }

    #[test]
    fn test_base_hp_mismatch_starred() {
        let pile_of_guts = Stage::from_id_current(StageID::from_components(T::UL, 31, 5)).unwrap();
        assert_eq!(pile_of_guts.base_hp, 1_000_000);
        assert_eq!(
            base_hp(&pile_of_guts),
            vec![
                TemplateParameter::new("enemy castle hp", "1,200,000 HP".to_string()),
                TemplateParameter::new("enemy castle hp2", "1,560,000 HP".to_string()),
                TemplateParameter::new("enemy castle hp3", "2,040,000 HP".to_string()),
            ]
        );
        // As of 13.6 this is the only stage where base hp != actual stat and
        // also has 4 crowns.
    }

    #[test]
    fn test_misc_info() {
        let earthshaker = Stage::from_id_current(StageID::from_components(T::SoL, 0, 0)).unwrap();
        assert_eq!(earthshaker.xp, Some(950));
        assert_eq!(
            xp(&earthshaker),
            Some(TemplateParameter::new("XP", "950 XP".to_string()))
        );
        assert_eq!(earthshaker.width, 4_200);
        assert_eq!(
            width(&earthshaker),
            TemplateParameter::new("width", "4,200".to_string())
        );
        assert_eq!(earthshaker.max_enemies, 7);
        assert_eq!(
            max_enemies(&earthshaker),
            TemplateParameter::new("max enemies", "7".to_string())
        );

        let labyrinth_67 =
            Stage::from_id_current(StageID::from_components(T::Labyrinth, 0, 66)).unwrap();
        assert_eq!(labyrinth_67.xp, None);
        assert_eq!(xp(&labyrinth_67), None);
        assert_eq!(labyrinth_67.width, 3_900);
        assert_eq!(
            width(&labyrinth_67),
            TemplateParameter::new("width", "3,900".to_string())
        );
        assert_eq!(labyrinth_67.max_enemies, 30);
        assert_eq!(
            max_enemies(&labyrinth_67),
            TemplateParameter::new("max enemies", "30".to_string())
        );
    }
}
