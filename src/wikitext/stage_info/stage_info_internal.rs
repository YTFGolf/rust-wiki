use super::StageWikiData;
use crate::{
    data::stage::{
        parsed::{
            stage::Stage,
            stage_enemy::{BossType, StageEnemy},
        },
        stage_metadata::consts::StageTypeEnum,
    },
    wikitext::{
        data_files::enemy_data::ENEMY_DATA,
        template_parameter::TemplateParameter,
        wiki_utils::{extract_name, REGEXES},
    },
};
use either::Either::{Left, Right};
use num_format::{Locale, WriteFormatted};
use std::{collections::HashSet, io::Write};

pub fn enemies_appearing(stage: &Stage) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    buf.write(b"{{EnemiesAppearing").unwrap();

    let mut displayed = HashSet::new();
    let enemies = stage
        .enemies
        .iter()
        .filter(|e| e.id != 21 && displayed.insert(e.id));

    for enemy in enemies {
        write!(buf, "|{}", ENEMY_DATA.get_common_name(enemy.id)).unwrap();
    }
    buf.write(b"}}").unwrap();

    buf
}

pub fn intro(stage: &Stage, data: &StageWikiData) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    if stage.meta.type_enum == StageTypeEnum::RankingDojo {
        write!(
            buf,
            "'''{extracted_name}''' is the {num} [[Arena of Honor]] of the [[Catclaw Dojo]].",
            extracted_name = extract_name(&data.stage_name.name),
            num = get_ordinal(stage.meta.map_num + 1)
        )
        .unwrap();

        return buf;
    }

    write!(
        buf,
        "'''{name}''' is the ",
        name = extract_name(&data.stage_name.name)
    )
    .unwrap();

    let num = stage.meta.stage_num;
    match (num, data.stage_map.get(num + 1)) {
        (0, None) => {
            buf.write(b"only").unwrap();
        }
        (n, next) => {
            write!(
                buf,
                "{ord}{is_last}",
                ord = get_ordinal(n + 1),
                is_last = match next {
                    None => " and final",
                    _ => "",
                }
            )
            .unwrap();
        }
    };
    // only/nth/nth and final

    write!(
        buf,
        " {stage_in} {map_name}{punct}",
        stage_in = match stage.meta.type_enum {
            StageTypeEnum::Tower => "floor of",
            _ => "stage in",
        },
        map_name = REGEXES
            .old_or_removed_sub
            .replace(&data.stage_map.name, "$1"),
        punct = match extract_name(&data.stage_map.name).chars().last().unwrap() {
            '!' | '.' => "",
            _ => ".",
        }
    )
    .unwrap();

    if stage.is_no_continues {
        buf.write(b" This is a [[No Continues]] stage.").unwrap();
    }

    buf
}

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

pub fn enemies_list(stage: &Stage) -> Vec<TemplateParameter> {
    struct EnemyListWithDupes<'a> {
        base: Vec<&'a StageEnemy>,
        enemies: Vec<&'a StageEnemy>,
        boss: Vec<&'a StageEnemy>,
    }
    let anim_base_id = stage.anim_base_id.map(|i| u32::from(i)).unwrap_or(1);

    let mut enemy_list = EnemyListWithDupes {
        base: vec![],
        enemies: vec![],
        boss: vec![],
    };
    for enemy in stage.enemies.iter() {
        if enemy.id + 2 == anim_base_id {
            enemy_list.base.push(enemy);
        } else if enemy.boss_type == BossType::None {
            enemy_list.enemies.push(enemy);
        } else {
            enemy_list.boss.push(enemy);
        }
    }
    // get all enemies

    assert!(enemy_list.base.len() <= 1);
    let mut enemy_list_seen = HashSet::new();
    let filtered_enemies = enemy_list
        .enemies
        .into_iter()
        .filter(|e| e.id != 21 && enemy_list_seen.insert((e.id, e.magnification)))
        .collect::<Vec<&StageEnemy>>();
    let mut boss_list_seen = HashSet::new();
    let filtered_boss = enemy_list
        .boss
        .into_iter()
        .filter(|e| e.id != 21 && boss_list_seen.insert((e.id, e.magnification)))
        .collect::<Vec<&StageEnemy>>();
    // remove duplicates

    /// Write `|{enemy}|{mag}%` to `buf`. Multiplier is raw % i.e. 100 = *1.
    fn write_enemy(buf: &mut Vec<u8>, enemy: &StageEnemy, multiplier: u32) {
        write!(buf, "|{}|", ENEMY_DATA.get_common_name(enemy.id)).unwrap();
        match &enemy.magnification {
            Left(m) => {
                buf.write_formatted(&(m * multiplier / 100), &Locale::en)
                    .unwrap();
                buf.write(b"%").unwrap();
            }
            Right((hp, ap)) => {
                buf.write_formatted(&(hp * multiplier / 100), &Locale::en)
                    .unwrap();
                buf.write(b"/").unwrap();
                buf.write_formatted(&(ap * multiplier / 100), &Locale::en)
                    .unwrap();
                buf.write(b"%").unwrap();
            }
        };
    }
    /// Collect all enemies in the vec to a newline-separated byte string.
    /// Multiplier is raw % i.e. 100 = *1.
    fn collect_all_enemies<'a>(
        filtered_enemies_vec: &Vec<&'a StageEnemy>,
        multiplier: u32,
    ) -> Vec<u8> {
        filtered_enemies_vec
            .iter()
            .map(|e| {
                let mut buf = vec![];
                write_enemy(&mut buf, e, multiplier);
                buf
            })
            .collect::<Vec<Vec<u8>>>()
            .join(&b'\n')
    }
    // util functions

    let mut enemy_vec: Vec<TemplateParameter> = vec![];
    let mut add_to_enemy_vec = |key: &'static [u8], list: Vec<u8>| {
        let mut buf = vec![];
        buf.write(b"{{Magnification").unwrap();
        buf.extend(list);
        buf.write(b"}}").unwrap();

        enemy_vec.push(TemplateParameter::new(key, buf));
    };
    // return value and another util function (has to be a mutable closure
    // since it uses `enemy_vec`).

    if !enemy_list.base.is_empty() {
        let base_items = collect_all_enemies(&enemy_list.base, 100);
        add_to_enemy_vec(b"base", base_items);
    }
    if !filtered_enemies.is_empty() {
        let enemy_items = collect_all_enemies(&filtered_enemies, 100);
        add_to_enemy_vec(b"enemies", enemy_items);
    }
    if !filtered_boss.is_empty() {
        let boss_items = collect_all_enemies(&filtered_boss, 100);
        add_to_enemy_vec(b"boss", boss_items);
    }

    let crowns = match &stage.crown_data {
        None => return enemy_vec,
        Some(c) => c,
    };
    let difficulty: u8 = crowns.max_difficulty.into();
    if difficulty == 1 {
        return enemy_vec;
    }

    let magnif_2: u32 = crowns.crown_2.unwrap().into();
    if !enemy_list.base.is_empty() {
        let base_items = collect_all_enemies(&enemy_list.base, magnif_2);
        add_to_enemy_vec(b"base2", base_items);
    }
    if !filtered_enemies.is_empty() {
        let enemy_items = collect_all_enemies(&filtered_enemies, magnif_2);
        add_to_enemy_vec(b"enemies2", enemy_items);
    }
    if !filtered_boss.is_empty() {
        let boss_items = collect_all_enemies(&filtered_boss, magnif_2);
        add_to_enemy_vec(b"boss2", boss_items);
    }
    if difficulty == 2 {
        return enemy_vec;
    }

    let magnif_3: u32 = crowns.crown_3.unwrap().into();
    if !enemy_list.base.is_empty() {
        let base_items = collect_all_enemies(&enemy_list.base, magnif_3);
        add_to_enemy_vec(b"base3", base_items);
    }
    if !filtered_enemies.is_empty() {
        let enemy_items = collect_all_enemies(&filtered_enemies, magnif_3);
        add_to_enemy_vec(b"enemies3", enemy_items);
    }
    if !filtered_boss.is_empty() {
        let boss_items = collect_all_enemies(&filtered_boss, magnif_3);
        add_to_enemy_vec(b"boss3", boss_items);
    }
    if difficulty == 3 {
        return enemy_vec;
    }

    let magnif_4: u32 = crowns.crown_4.unwrap().into();
    if magnif_4 == 100 {
        return enemy_vec;
    }
    if !enemy_list.base.is_empty() {
        let base_items = collect_all_enemies(&enemy_list.base, magnif_4);
        add_to_enemy_vec(b"base4", base_items);
    }
    if !filtered_enemies.is_empty() {
        let enemy_items = collect_all_enemies(&filtered_enemies, magnif_4);
        add_to_enemy_vec(b"enemies4", enemy_items);
    }
    if !filtered_boss.is_empty() {
        let boss_items = collect_all_enemies(&filtered_boss, magnif_4);
        add_to_enemy_vec(b"boss4", boss_items);
    }
    // TODO disable for gauntlets/dojo

    enemy_vec
}

pub fn restrictions_section(_stage: &Stage) -> Vec<u8> {
    vec![]
}

fn get_ordinal(n: u32) -> String {
    const SMALL_ORDS: [&str; 9] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
    ];

    if n as usize <= SMALL_ORDS.len() {
        return SMALL_ORDS[n as usize - 1].to_string();
    }

    let n = n % 100;
    if (11..=13).contains(&n) {
        format!("{n}th")
    } else if n % 10 == 1 {
        format!("{n}st")
    } else if n % 10 == 2 {
        format!("{n}nd")
    } else if n % 10 == 3 {
        format!("{n}rd")
    } else {
        format!("{n}th")
    }
}

pub fn param_vec_fold(mut buf: Vec<u8>, param: TemplateParameter) -> Vec<u8> {
    let smallbuf = param.to_u8s();
    buf.extend(smallbuf.iter());
    buf.write(b"\n").unwrap();
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wikitext::data_files::stage_names::STAGE_NAMES;
    // TODO split all of these up properly.

    fn get_stage_wiki_data(stage: &Stage) -> StageWikiData {
        let stage_map = STAGE_NAMES
            .stage_map(stage.meta.type_num, stage.meta.map_num)
            .unwrap();
        let stage_name = stage_map.get(stage.meta.stage_num).unwrap();
        StageWikiData {
            stage_map,
            stage_name,
        }
    }

    #[test]
    fn test_intro() {
        let ht30 = Stage::new("v 0 29").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&ht30);
        let buf = intro(&ht30, &stage_wiki_data);
        assert_eq!(&String::from_utf8(buf).unwrap(), "'''Floor 30''' is the 30th floor of [[Heavenly Tower]]. This is a [[No Continues]] stage.");

        let whole_new = Stage::new("zl 0 0").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&whole_new);
        let buf = intro(&whole_new, &stage_wiki_data);
        assert_eq!(&String::from_utf8(buf).unwrap(), "'''A Whole New World''' is the only stage in [[Zero Field]]. This is a [[No Continues]] stage.");

        let earthshaker = Stage::new("sol 0 0").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&earthshaker);
        let buf = intro(&earthshaker, &stage_wiki_data);
        assert_eq!(
            &String::from_utf8(buf).unwrap(),
            "'''Earthshaker''' is the first stage in [[The Legend Begins]]."
        );

        let refusal_type = Stage::new("c 206 1").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&refusal_type);
        let buf = intro(&refusal_type, &stage_wiki_data);
        assert_eq!(&String::from_utf8(buf).unwrap(), "'''Refusal Type (Merciless)''' is the second and final stage in [[The 10th Angel Strikes!]] This is a [[No Continues]] stage.");

        let crimson_trial = Stage::new("r 20 0").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&crimson_trial);
        let buf = intro(&crimson_trial, &stage_wiki_data);
        assert_eq!(
            &String::from_utf8(buf).unwrap(),
            "'''Crimson Trial''' is the 21st [[Arena of Honor]] of the [[Catclaw Dojo]]."
        );
    }

    #[test]
    fn test_enemies_appearing() {
        let crazed_cat = Stage::new("s 17 0").unwrap();
        let buf = enemies_appearing(&crazed_cat);
        assert_eq!(
            &String::from_utf8(buf).unwrap(),
            "{{EnemiesAppearing|Le'boin|Teacher Bear|Doge|Snache|Croco|Crazed Cat}}"
        );

        let tada = Stage::new("ex 63 0").unwrap();
        let buf = enemies_appearing(&tada);
        assert_eq!(&String::from_utf8(buf).unwrap(), "{{EnemiesAppearing}}");

        let not_alone = Stage::new("c 176 4").unwrap();
        let buf = enemies_appearing(&not_alone);
        assert_eq!(&String::from_utf8(buf).unwrap(), "{{EnemiesAppearing|Shibalien|Mistress Celeboodle|Imperator Sael|Kroxo|Cyberhorn|Charlotte (Snake)}}");
        // Star Ocean
    }

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
    fn test_energy() {
        let aac = Stage::new("ul 0 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(energy(&aac).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = 200");

        let challenge = Stage::new("challenge 0 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(energy(&challenge).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = 0");

        let door_opens = Stage::new("ex 47 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(energy(&door_opens).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = N/A");

        let facing_danger = Stage::new("b 5 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(energy(&facing_danger).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = N/A");

        let mining_epic = Stage::new("s 326 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(energy(&mining_epic).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = 1,000");

        let labyrinth_67 = Stage::new("l 0 66").unwrap();
        assert_eq!(energy(&labyrinth_67), None);
    }

    #[test]
    fn test_base_hp() {
        let ht30 = Stage::new("v 0 29").unwrap();
        assert_eq!(
            base_hp(&ht30),
            vec![TemplateParameter::new(
                b"enemy castle hp",
                b"1,000,000 HP".to_vec()
            )]
        );

        let dojo = Stage::new("t 0 0").unwrap();
        assert_eq!(
            base_hp(&dojo),
            vec![TemplateParameter::new(
                b"enemy castle hp",
                b"Unlimited".to_vec()
            )]
        );

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

    #[test]
    fn test_enemies_list() {
        let aac = Stage::new("ul 0 0").unwrap();
        assert_eq!(
            enemies_list(&aac),
            vec![
                TemplateParameter::new(b"enemies", b"{{Magnification|Relic Doge|100%}}".to_vec()),
                TemplateParameter::new(b"boss", b"{{Magnification|Relic Bun-Bun|100%}}".to_vec()),
                TemplateParameter::new(b"enemies2", b"{{Magnification|Relic Doge|150%}}".to_vec()),
                TemplateParameter::new(b"boss2", b"{{Magnification|Relic Bun-Bun|150%}}".to_vec()),
                TemplateParameter::new(b"enemies3", b"{{Magnification|Relic Doge|200%}}".to_vec()),
                TemplateParameter::new(b"boss3", b"{{Magnification|Relic Bun-Bun|200%}}".to_vec()),
            ]
        );

        let tada = Stage::new("ex 63 0").unwrap();
        assert_eq!(enemies_list(&tada), vec![]);

        let celestial_seas = Stage::new("n 32 3").unwrap();
        assert_eq!(
            enemies_list(&celestial_seas),
            vec![
                TemplateParameter::new(
                    b"enemies",
                    b"{{Magnification|Doge|3,000%\n\
                    |Those Guys|2,000%\n\
                    |Gabriel|400%\n\
                    |Gabriel|600%\n\
                    |Gabriel|700%\n\
                    |Gabriel|800%\n\
                    |Gabriel|900%\n\
                    |Gabriel|1,000%\n\
                    |Gabriel|2,000%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss", b"{{Magnification|Le'boin|10,000%}}".to_vec()),
                TemplateParameter::new(
                    b"enemies2",
                    b"{{Magnification|Doge|3,600%\n\
                    |Those Guys|2,400%\n\
                    |Gabriel|480%\n\
                    |Gabriel|720%\n\
                    |Gabriel|840%\n\
                    |Gabriel|960%\n\
                    |Gabriel|1,080%\n\
                    |Gabriel|1,200%\n\
                    |Gabriel|2,400%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss2", b"{{Magnification|Le'boin|12,000%}}".to_vec()),
                TemplateParameter::new(
                    b"enemies3",
                    b"{{Magnification|Doge|4,200%\n\
                    |Those Guys|2,800%\n\
                    |Gabriel|560%\n\
                    |Gabriel|840%\n\
                    |Gabriel|980%\n\
                    |Gabriel|1,120%\n\
                    |Gabriel|1,260%\n\
                    |Gabriel|1,400%\n\
                    |Gabriel|2,800%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss3", b"{{Magnification|Le'boin|14,000%}}".to_vec()),
            ]
        );

        let it_25 = Stage::new("v 6 24").unwrap();
        assert_eq!(
            enemies_list(&it_25),
            vec![TemplateParameter::new(
                b"enemies",
                b"{{Magnification|Pigeon de Sable|300%\n\
                |Elizabeth the LVIth|2,000%\n\
                |Bore Jr.|100%\n\
                |Kory|600%\n\
                |Berserkory|200%\n\
                |Heavy Assault C.A.T.|100/150%\n\
                |Mr. Angel|300%}}"
                    .to_vec()
            )]
        );

        let sacrifice_apprenticeship = Stage::new("nd 3 3").unwrap();
        assert_eq!(
            enemies_list(&sacrifice_apprenticeship),
            vec![
                TemplateParameter::new(
                    b"enemies",
                    b"{{Magnification|Celeboodle|1,000%\n\
                    |Relic Doge|150%\n\
                    |Sir Rel|150%}}"
                        .to_vec()
                ),
                TemplateParameter::new(
                    b"boss",
                    b"{{Magnification|Ururun Wolf|300/500%\n\
                    |Mystic Mask Yulala|100%}}"
                        .to_vec()
                )
            ]
        );

        let sleeping_lion = Stage::new("sol 0 7").unwrap();
        assert_eq!(
            enemies_list(&sleeping_lion),
            vec![
                TemplateParameter::new(
                    b"enemies",
                    b"{{Magnification|Doge|400%\n\
                    |Snache|400%\n\
                    |Those Guys|400%\n\
                    |Gory|400%\n\
                    |Hippoe|400%\n\
                    |Doge Dark|100%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss", b"{{Magnification|Squire Rel|100%}}".to_vec()),
                TemplateParameter::new(
                    b"enemies2",
                    b"{{Magnification|Doge|600%\n\
                    |Snache|600%\n\
                    |Those Guys|600%\n\
                    |Gory|600%\n\
                    |Hippoe|600%\n\
                    |Doge Dark|150%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss2", b"{{Magnification|Squire Rel|150%}}".to_vec()),
                TemplateParameter::new(
                    b"enemies3",
                    b"{{Magnification|Doge|800%\n\
                    |Snache|800%\n\
                    |Those Guys|800%\n\
                    |Gory|800%\n\
                    |Hippoe|800%\n\
                    |Doge Dark|200%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss3", b"{{Magnification|Squire Rel|200%}}".to_vec()),
                TemplateParameter::new(
                    b"enemies4",
                    b"{{Magnification|Doge|1,200%\n\
                    |Snache|1,200%\n\
                    |Those Guys|1,200%\n\
                    |Gory|1,200%\n\
                    |Hippoe|1,200%\n\
                    |Doge Dark|300%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss4", b"{{Magnification|Squire Rel|300%}}".to_vec()),
            ]
        );

        let star_ocean = Stage::new("sol 15 7").unwrap();
        assert_eq!(
            enemies_list(&star_ocean),
            [
                TemplateParameter::new(
                    b"enemies",
                    b"{{Magnification|Doge|2,000%\n\
                    |Those Guys|400%\n\
                    |Doge Dark|400%\n\
                    |Doge Dark|500%\n\
                    |Doge Dark|600%\n\
                    |Doge Dark|800%\n\
                    |Doge Dark|1,000%\n\
                    |Doge Dark|1,200%\n\
                    |Doge Dark|2,000%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss", b"{{Magnification|H. Nah|200%}}".to_vec()),
                TemplateParameter::new(
                    b"enemies2",
                    b"{{Magnification|Doge|3,000%\n\
                    |Those Guys|600%\n\
                    |Doge Dark|600%\n\
                    |Doge Dark|750%\n\
                    |Doge Dark|900%\n\
                    |Doge Dark|1,200%\n\
                    |Doge Dark|1,500%\n\
                    |Doge Dark|1,800%\n\
                    |Doge Dark|3,000%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss2", b"{{Magnification|H. Nah|300%}}".to_vec()),
                TemplateParameter::new(
                    b"enemies3",
                    b"{{Magnification|Doge|4,000%\n\
                    |Those Guys|800%\n\
                    |Doge Dark|800%\n\
                    |Doge Dark|1,000%\n\
                    |Doge Dark|1,200%\n\
                    |Doge Dark|1,600%\n\
                    |Doge Dark|2,000%\n\
                    |Doge Dark|2,400%\n\
                    |Doge Dark|4,000%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss3", b"{{Magnification|H. Nah|400%}}".to_vec()),
                TemplateParameter::new(
                    b"enemies4",
                    b"{{Magnification|Doge|4,000%\n\
                    |Those Guys|800%\n\
                    |Doge Dark|800%\n\
                    |Doge Dark|1,000%\n\
                    |Doge Dark|1,200%\n\
                    |Doge Dark|1,600%\n\
                    |Doge Dark|2,000%\n\
                    |Doge Dark|2,400%\n\
                    |Doge Dark|4,000%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss4", b"{{Magnification|H. Nah|400%}}".to_vec()),
            ]
        );

        let kugel_schreiber = Stage::new("sol 24 2").unwrap();
        assert_eq!(
            enemies_list(&kugel_schreiber),
            vec![
                TemplateParameter::new(
                    b"enemies",
                    b"{{Magnification|Assassin Bear|200%}}".to_vec()
                ),
                TemplateParameter::new(
                    b"boss",
                    b"{{Magnification|Dober P.D|100%\n\
                    |R.Ost|100%\n\
                    |THE SLOTH|200%}}"
                        .to_vec()
                ),
                TemplateParameter::new(
                    b"enemies2",
                    b"{{Magnification|Assassin Bear|240%}}".to_vec()
                ),
                TemplateParameter::new(
                    b"boss2",
                    b"{{Magnification|Dober P.D|120%\n\
                    |R.Ost|120%\n\
                    |THE SLOTH|240%}}"
                        .to_vec()
                ),
                TemplateParameter::new(
                    b"enemies3",
                    b"{{Magnification|Assassin Bear|280%}}".to_vec()
                ),
                TemplateParameter::new(
                    b"boss3",
                    b"{{Magnification|Dober P.D|140%\n\
                    |R.Ost|140%\n\
                    |THE SLOTH|280%}}"
                        .to_vec()
                ),
                TemplateParameter::new(
                    b"enemies4",
                    b"{{Magnification|Assassin Bear|220%}}".to_vec()
                ),
                TemplateParameter::new(
                    b"boss4",
                    b"{{Magnification|Dober P.D|110%\n\
                    |R.Ost|110%\n\
                    |THE SLOTH|220%}}"
                        .to_vec()
                )
            ]
        );

        let noble_tribe = Stage::new("sol 43 2").unwrap();
        assert_eq!(
            enemies_list(&noble_tribe),
            vec![
                TemplateParameter::new(
                    b"enemies",
                    b"{{Magnification|Doge|120,000%\n\
                    |Snache|120,000%\n\
                    |Those Guys|120,000%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss", b"{{Magnification|Hippoe|120,000%}}".to_vec()),
                TemplateParameter::new(
                    b"enemies2",
                    b"{{Magnification|Doge|144,000%\n\
                    |Snache|144,000%\n\
                    |Those Guys|144,000%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss2", b"{{Magnification|Hippoe|144,000%}}".to_vec()),
                TemplateParameter::new(
                    b"enemies3",
                    b"{{Magnification|Doge|156,000%\n\
                    |Snache|156,000%\n\
                    |Those Guys|156,000%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss3", b"{{Magnification|Hippoe|156,000%}}".to_vec()),
            ]
        );

        let revenant_road = Stage::new("sol 33 3").unwrap();
        assert_eq!(
            enemies_list(&revenant_road),
            vec![
                TemplateParameter::new(
                    b"enemies",
                    b"{{Magnification|Zroco|200%\n\
                    |Zir Zeal|200%\n\
                    |Zigge|200%\n\
                    |Zomboe|200%\n\
                    |B.B.Bunny|2,800%}}"
                        .to_vec()
                ),
                TemplateParameter::new(
                    b"boss",
                    b"{{Magnification|Teacher Bun Bun|1,500%}}".to_vec()
                ),
                TemplateParameter::new(
                    b"enemies2",
                    b"{{Magnification|Zroco|240%\n\
                    |Zir Zeal|240%\n\
                    |Zigge|240%\n\
                    |Zomboe|240%\n\
                    |B.B.Bunny|3,360%}}"
                        .to_vec()
                ),
                TemplateParameter::new(
                    b"boss2",
                    b"{{Magnification|Teacher Bun Bun|1,800%}}".to_vec()
                ),
                TemplateParameter::new(
                    b"enemies3",
                    b"{{Magnification|Zroco|280%\n\
                    |Zir Zeal|280%\n\
                    |Zigge|280%\n\
                    |Zomboe|280%\n\
                    |B.B.Bunny|3,920%}}"
                        .to_vec()
                ),
                TemplateParameter::new(
                    b"boss3",
                    b"{{Magnification|Teacher Bun Bun|2,100%}}".to_vec()
                ),
            ]
        );

        let finale = Stage::new("c 209 0").unwrap();
        assert_eq!(
            enemies_list(&finale),
            vec![TemplateParameter::new(
                b"base",
                b"{{Magnification|Finale Base|100%}}".to_vec()
            ),]
        );

        let relay_1600m = Stage::new("ex 61 2").unwrap();
        assert_eq!(
            enemies_list(&relay_1600m),
            vec![
                TemplateParameter::new(
                    b"base",
                    b"{{Magnification|Relay Base|7,500,000%}}".to_vec()
                ),
                TemplateParameter::new(
                    b"enemies",
                    b"{{Magnification|White Wind|700%\n\
                    |Duche|300%\n\
                    |Red Wind|700%\n\
                    |Gory Black|200%\n\
                    |Black Wind|700%\n\
                    |R.Ost|100%\n\
                    |Bore|200%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss", b"{{Magnification|Le'noir|150%}}".to_vec()),
            ]
        );

        let pile_of_guts = Stage::new("ul 31 5").unwrap();
        assert_eq!(
            enemies_list(&pile_of_guts),
            vec![
                TemplateParameter::new(
                    b"base",
                    b"{{Magnification|Relic Doge Base|40,000%}}".to_vec()
                ),
                TemplateParameter::new(
                    b"enemies",
                    b"{{Magnification|Bore Jr.|100%\n\
                    |Celeboodle|1,000%\n\
                    |R.Ost|300%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss", b"{{Magnification|THE SLOTH|400%}}".to_vec()),
                TemplateParameter::new(
                    b"base2",
                    b"{{Magnification|Relic Doge Base|52,000%}}".to_vec()
                ),
                TemplateParameter::new(
                    b"enemies2",
                    b"{{Magnification|Bore Jr.|130%\n\
                    |Celeboodle|1,300%\n\
                    |R.Ost|390%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss2", b"{{Magnification|THE SLOTH|520%}}".to_vec()),
                TemplateParameter::new(
                    b"base3",
                    b"{{Magnification|Relic Doge Base|68,000%}}".to_vec()
                ),
                TemplateParameter::new(
                    b"enemies3",
                    b"{{Magnification|Bore Jr.|170%\n\
                    |Celeboodle|1,700%\n\
                    |R.Ost|510%}}"
                        .to_vec()
                ),
                TemplateParameter::new(b"boss3", b"{{Magnification|THE SLOTH|680%}}".to_vec()),
            ]
        );

        // println!("{:?}", enemies_list(&it_25).into_iter().map(String::from).collect::<Vec<_>>());

        /*
        import re
        def get_lines():
            lines = input('Input things: ')
            new = 1
            while new:
                new = input()
                lines = f'{lines}\n{new}'
            return lines[:-1]

        lines = get_lines()
        for key, value in re.findall(r'\|(\w+) = (\{\{(?:.|\n)*?\}\})', lines):
            value = value.replace('%\n|', "%\\n\\" + f"\n{' ' * 20}|")
            print(f'TemplateParameter::new(b"{key}", b"{value}".to_vec()),')
        */
    }

    // Should probably also test param_vec_fold
}
