//! Prints information about a stage.

use super::{
    data_files::{
        enemy_data::ENEMY_DATA,
        stage_names::{MapData, StageData},
    },
    template_parameter::TemplateParameter,
    wiki_utils::{extract_name, REGEXES},
};
use crate::{
    data::stage::{parsed::stage::Stage, stage_metadata::consts::StageTypeEnum},
    wikitext::{
        data_files::stage_names::STAGE_NAMES,
        format_parser::{parse_si_format, ParseType},
    },
};
use num_format::{Locale, WriteFormatted};
use regex::Regex;
use std::{collections::HashSet, io::Write};

const DEFAULT_FORMAT: &str = "\
${enemies_appearing}
${intro}

{{Stage Info
${stage_name}
${stage_location}
${energy}
${base_hp}
${enemies_list}
${treasure}
${restrictions_info}
${score_reward}
${xp}
${width}
${max_enemies}
|jpname = ?\n|script = ?\n|romaji = ?
${star}
${chapter}
${difficulty}
${stage_nav}
}}

==Restrictions==
${restrictions_section}

==Battleground==
${battlegrounds}

==Strategy==
-

==Reference==
*${reference}\
";

struct StageWikiData {
    stage_map: &'static MapData,
    stage_name: &'static StageData,
}

fn do_thing_internal() {
    // Something to do with the old pre and post
    let format = DEFAULT_FORMAT;
    let parsed = parse_si_format(format);

    let mut buf = vec![];
    let stage = Stage::new("v 0 29").unwrap();

    let stage_map = STAGE_NAMES
        .stage_map(stage.meta.type_num, stage.meta.map_num)
        .unwrap();
    let stage_name = stage_map.get(stage.meta.stage_num).unwrap();

    let stage_wiki_data = StageWikiData {
        stage_map,
        stage_name,
    };

    for node in parsed {
        if node.ptype == ParseType::Text {
            buf.write(node.content.as_bytes()).unwrap();
            continue;
        }

        let new_buf = match node.content {
            "enemies_appearing" => StageInfo::enemies_appearing(&stage),
            "intro" => StageInfo::intro(&stage, &stage_wiki_data),
            "stage_name" => StageInfo::stage_name(&stage).to_u8s(),
            "stage_location" => StageInfo::stage_location(&stage).to_u8s(),
            "energy" => StageInfo::energy(&stage)
                .map(|tp| tp.to_u8s())
                .unwrap_or(b"".to_vec()),
            "restrictions_section" => StageInfo::restrictions_section(&stage),

            _ => continue,
        };
        buf.extend(new_buf.into_iter());
    }

    let string_buf = String::from_utf8(buf).unwrap();
    let string_buf = Regex::new(r"\n+(\||\}\})")
        .unwrap()
        .replace_all(&string_buf, "\n$1");
    let string_buf = Regex::new(r"\n==.*==\n\n")
        .unwrap()
        .replace_all(&string_buf, "");

    println!("{}", string_buf);
}

/// temp
pub fn do_stuff() {
    do_thing_internal()
    // println!("{DEFAULT_FORMAT:?}");
}

struct StageInfo;
impl StageInfo {
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

    pub fn restrictions_section(_stage: &Stage) -> Vec<u8> {
        vec![]
    }
}

// No context stage enemy line, just takes &enemy and show_mag. if base then can
// just write it in the function directly, and ** can be written there too.

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

#[cfg(test)]
mod tests {
    use super::*;

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
        let buf = StageInfo::intro(&ht30, &stage_wiki_data);
        assert_eq!(&String::from_utf8(buf).unwrap(), "'''Floor 30''' is the 30th floor of [[Heavenly Tower]]. This is a [[No Continues]] stage.");

        let whole_new = Stage::new("zl 0 0").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&whole_new);
        let buf = StageInfo::intro(&whole_new, &stage_wiki_data);
        assert_eq!(&String::from_utf8(buf).unwrap(), "'''A Whole New World''' is the only stage in [[Zero Field]]. This is a [[No Continues]] stage.");

        let earthshaker = Stage::new("sol 0 0").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&earthshaker);
        let buf = StageInfo::intro(&earthshaker, &stage_wiki_data);
        assert_eq!(
            &String::from_utf8(buf).unwrap(),
            "'''Earthshaker''' is the first stage in [[The Legend Begins]]."
        );

        let refusal_type = Stage::new("c 206 1").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&refusal_type);
        let buf = StageInfo::intro(&refusal_type, &stage_wiki_data);
        assert_eq!(&String::from_utf8(buf).unwrap(), "'''Refusal Type (Merciless)''' is the second and final stage in [[The 10th Angel Strikes!]] This is a [[No Continues]] stage.");

        let crimson_trial = Stage::new("r 20 0").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&crimson_trial);
        let buf = StageInfo::intro(&crimson_trial, &stage_wiki_data);
        assert_eq!(
            &String::from_utf8(buf).unwrap(),
            "'''Crimson Trial''' is the 21st [[Arena of Honor]] of the [[Catclaw Dojo]]."
        );
    }

    #[test]
    fn test_enemies_appearing() {
        let crazed_cat = Stage::new("s 17 0").unwrap();
        let buf = StageInfo::enemies_appearing(&crazed_cat);
        assert_eq!(
            &String::from_utf8(buf).unwrap(),
            "{{EnemiesAppearing|Le'boin|Teacher Bear|Doge|Snache|Croco|Crazed Cat}}"
        );

        let tada = Stage::new("ex 63 0").unwrap();
        let buf = StageInfo::enemies_appearing(&tada);
        assert_eq!(&String::from_utf8(buf).unwrap(), "{{EnemiesAppearing}}");

        let not_alone = Stage::new("c 176 4").unwrap();
        let buf = StageInfo::enemies_appearing(&not_alone);
        assert_eq!(&String::from_utf8(buf).unwrap(), "{{EnemiesAppearing|Shibalien|Mistress Celeboodle|Imperator Sael|Kroxo|Cyberhorn|Charlotte (Snake)}}");
    }

    #[test]
    fn test_stage_name_and_loc() {
        let great_escaper = Stage::new("n 17 5").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(StageInfo::stage_name(&great_escaper).to_u8s());
        buf.write(b"\n").unwrap();
        buf.extend(StageInfo::stage_location(&great_escaper).to_u8s());
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
        buf.extend(StageInfo::stage_name(&red_summit).to_u8s());
        buf.write(b"\n").unwrap();
        buf.extend(StageInfo::stage_location(&red_summit).to_u8s());
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
        buf.extend(StageInfo::stage_name(&finale).to_u8s());
        buf.write(b"\n").unwrap();
        buf.extend(StageInfo::stage_location(&finale).to_u8s());
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
        buf.extend(StageInfo::stage_name(&relay_1600m).to_u8s());
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
        buf.extend(StageInfo::energy(&aac).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = 200");

        let challenge = Stage::new("challenge 0 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(StageInfo::energy(&challenge).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = 0");

        let door_opens = Stage::new("ex 47 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(StageInfo::energy(&door_opens).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = N/A");

        let facing_danger = Stage::new("b 5 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(StageInfo::energy(&facing_danger).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = N/A");

        let mining_epic = Stage::new("s 326 0").unwrap();
        let mut buf: Vec<u8> = vec![];
        buf.extend(StageInfo::energy(&mining_epic).unwrap().to_u8s());
        assert_eq!(&String::from_utf8(buf).unwrap(), "|energy = 1,000");

        let labyrinth_67 = Stage::new("l 0 66").unwrap();
        assert_eq!(StageInfo::energy(&labyrinth_67), None);
    }
}
