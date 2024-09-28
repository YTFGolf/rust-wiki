use crate::{
    data::stage::{parsed::stage::Stage, stage_metadata::consts::StageTypeEnum},
    wikitext::{data_files::enemy_data::ENEMY_DATA, template_parameter::TemplateParameter},
};
use either::Either::{Left, Right};
use num_format::{Locale, WriteFormatted};
use std::io::Write;

pub fn stage_name(stage: &Stage) -> TemplateParameter {
    let mut buf: Vec<u8> = vec![];

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

    write!(
        buf,
        "\n[[File:Mapsn{map_num:03} {stage_num:02} {type_code} en.png]]",
        map_num = stage.meta.map_num,
        stage_num = stage.meta.stage_num,
        type_code = stage.meta.type_code.to_lowercase(),
    )
    .unwrap();

    TemplateParameter::new(b"stage name", buf)
}

pub fn stage_location(stage: &Stage) -> TemplateParameter {
    let mut buf = vec![];
    write!(
        &mut buf,
        "[[File:Mapname{map_num:03} {type_code} en.png]]",
        map_num = stage.meta.map_num,
        type_code = stage.meta.type_code.to_lowercase(),
    )
    .unwrap();
    TemplateParameter::new(b"stage location", buf)
}

pub fn energy(stage: &Stage) -> Option<TemplateParameter> {
    let energy = stage.energy?;
    let mut buf = vec![];
    match stage.meta.type_enum {
        StageTypeEnum::Catamin | StageTypeEnum::Extra => {
            let _ = buf.write(b"N/A").unwrap();
        }
        _ => {
            let _ = buf.write_formatted(&energy, &Locale::en).unwrap();
        }
    };

    Some(TemplateParameter::new(b"energy", buf))
}

pub fn base_hp(stage: &Stage) -> Vec<TemplateParameter> {
    const PARAM_NAME: &[u8] = b"enemy castle hp";
    const PARAM_NAME_2: &[u8] = b"enemy castle hp2";
    const PARAM_NAME_3: &[u8] = b"enemy castle hp3";
    const PARAM_NAME_4: &[u8] = b"enemy castle hp4";

    if stage.time_limit.is_some() {
        return vec![TemplateParameter::new(PARAM_NAME, b"Unlimited".to_vec())];
    }
    if stage.anim_base_id.is_none() {
        let mut buf = vec![];
        buf.write_formatted(&stage.base_hp, &Locale::en).unwrap();
        buf.write(b" HP").unwrap();
        return vec![TemplateParameter::new(PARAM_NAME, buf)];
    }

    let anim_base_id = <u32>::from(stage.anim_base_id.unwrap()) - 2;
    let hp = ENEMY_DATA.get_data(anim_base_id).hp;
    let mag_either = || {
        for enemy in &stage.enemies {
            if enemy.id == anim_base_id {
                return enemy.magnification;
            }
        }
        unreachable!()
    };
    let mag = match mag_either() {
        Left(m) => m,
        Right((hp, _ap)) => hp,
    };

    let magnification_hp = mag * hp / 100;
    if stage.crown_data.is_none() {
        let mut buf = vec![];
        buf.write_formatted(&magnification_hp, &Locale::en).unwrap();
        buf.write(b" HP").unwrap();
        return vec![TemplateParameter::new(PARAM_NAME, buf)];
    }

    let mut params = vec![];
    let get_new_param = |key, value| {
        let mut buf = vec![];
        buf.write_formatted(&value, &Locale::en).unwrap();
        buf.write(b" HP").unwrap();
        TemplateParameter::new(key, buf)
    };

    if let Some(crown_data) = &stage.crown_data {
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
        }
    }

    params
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_name_and_loc() {
        let great_escaper = Stage::new("n 17 5").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(stage_name(&great_escaper).to_u8s());
        buf.write(b"\n").unwrap();
        buf.extend(stage_location(&great_escaper).to_u8s());
        assert_eq!(
            &String::from_utf8(buf).unwrap(),
            "\
            |stage name = [[File:rc006.png]]\n\
            [[File:Mapsn017 05 n en.png]]\n\
            |stage location = [[File:Mapname017 n en.png]]\
            "
        );

        let red_summit = Stage::new("h 10 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(stage_name(&red_summit).to_u8s());
        buf.write(b"\n").unwrap();
        buf.extend(stage_location(&red_summit).to_u8s());
        assert_eq!(
            &String::from_utf8(buf).unwrap(),
            "\
            |stage name = [[File:rc002.png]]\n\
            [[File:Mapsn010 00 h en.png]]\n\
            |stage location = [[File:Mapname010 h en.png]]\
            "
        );

        let finale = Stage::new("c 209 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(stage_name(&finale).to_u8s());
        buf.write(b"\n").unwrap();
        buf.extend(stage_location(&finale).to_u8s());
        assert_eq!(
            &String::from_utf8(buf).unwrap(),
            "\
            |stage name = [[File:E 651.png]]\n\
            [[File:Mapsn209 00 c en.png]]\n\
            |stage location = [[File:Mapname209 c en.png]]\
            "
        );

        let relay_1600m = Stage::new("ex 61 2").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(stage_name(&relay_1600m).to_u8s());
        assert_eq!(
            &String::from_utf8(buf).unwrap(),
            "\
            |stage name = [[File:E 657.png|250px]]\n\
            [[File:Mapsn061 02 ex en.png]]\
            "
        );
    }

    #[test]
    fn test_energy_normal() {
        let aac = Stage::new("ul 0 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(energy(&aac).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = 200");
    }

    #[test]
    fn test_energy_0() {
        let challenge = Stage::new("challenge 0 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(energy(&challenge).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = 0");
    }

    #[test]
    fn test_energy_ex() {
        let door_opens = Stage::new("ex 47 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(energy(&door_opens).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = N/A");
    }

    #[test]
    fn test_energy_catamin() {
        let facing_danger = Stage::new("b 5 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(energy(&facing_danger).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = N/A");
    }

    #[test]
    fn test_energy_1_000() {
        let mining_epic = Stage::new("s 326 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(energy(&mining_epic).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = 1,000");
    }

    #[test]
    fn test_energy_labyrinth() {
        let labyrinth_67 = Stage::new("l 0 66").unwrap();
        assert_eq!(energy(&labyrinth_67), None);
    }

    #[test]
    fn test_base_hp_normal() {
        let ht30 = Stage::new("v 0 29").unwrap();
        assert_eq!(
            base_hp(&ht30),
            vec![TemplateParameter::new(
                b"enemy castle hp",
                b"1,000,000 HP".to_vec()
            )]
        );
    }

    #[test]
    fn test_base_hp_dojo() {
        let dojo = Stage::new("t 0 0").unwrap();
        assert_eq!(
            base_hp(&dojo),
            vec![TemplateParameter::new(
                b"enemy castle hp",
                b"Unlimited".to_vec()
            )]
        );
    }

    #[test]
    fn test_base_hp_mismatch() {
        // where stage.base_hp != actual base hp
        let just_friends = Stage::new("s 302 2").unwrap();
        assert_eq!(just_friends.base_hp, 10);
        assert_eq!(
            base_hp(&just_friends),
            vec![TemplateParameter::new(
                b"enemy castle hp",
                b"30,000 HP".to_vec()
            )]
        );

        let finale = Stage::new("c 209 0").unwrap();
        assert_eq!(finale.base_hp, 1_000);
        assert_eq!(
            base_hp(&finale),
            vec![TemplateParameter::new(
                b"enemy castle hp",
                b"50 HP".to_vec()
            )]
        );
    }

    #[test]
    fn test_base_hp_starred() {
        let rongorongo = Stage::new("s 129 5").unwrap();
        assert_eq!(rongorongo.base_hp, 300_000);
        assert_eq!(
            base_hp(&rongorongo),
            vec![
                TemplateParameter::new(b"enemy castle hp", b"300,000 HP".to_vec()),
                TemplateParameter::new(b"enemy castle hp2", b"450,000 HP".to_vec()),
                TemplateParameter::new(b"enemy castle hp3", b"600,000 HP".to_vec()),
                TemplateParameter::new(b"enemy castle hp4", b"900,000 HP".to_vec()),
            ]
        );
    }

    #[test]
    fn test_base_hp_mismatch_starred() {
        let pile_of_guts = Stage::new("ul 31 5").unwrap();
        assert_eq!(pile_of_guts.base_hp, 1_000_000);
        assert_eq!(
            base_hp(&pile_of_guts),
            vec![
                TemplateParameter::new(b"enemy castle hp", b"1,200,000 HP".to_vec()),
                TemplateParameter::new(b"enemy castle hp2", b"1,560,000 HP".to_vec()),
                TemplateParameter::new(b"enemy castle hp3", b"2,040,000 HP".to_vec()),
            ]
        );
        // As of 13.6 this is the only stage where base hp != actual stat and
        // also has 4 crowns.

        // println!("{:?}",
        // base_hp(&just_friends).into_iter().map(|a|
        // String::from_utf8(a.to_u8s())).collect::<Vec<_>>());
    }
}
