//! Prints information about a stage.

#![allow(clippy::unused_io_amount)]
use crate::{
    data::stage::{parsed::stage::Stage, stage_metadata::consts::StageTypeEnum},
    wikitext::{
        data_files::stage_names::STAGE_NAMES,
        format_parser::{parse_si_format, ParseType},
    },
};
use std::io::Write;

use super::data_files::stage_names::{MapData, StageData};

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
// TODO add another thing that can invalidate some lines.

struct StageWikiData {
    stage_map: &'static MapData,
    stage_name: &'static StageData,
}

fn do_thing_internal() {
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

        match node.content {
            "enemies_appearing" => StageInfo::enemies_appearing(&mut buf, &stage),
            "intro" => StageInfo::intro(&mut buf, &stage, &stage_wiki_data),
            "restrictions_section" => StageInfo::restrictions_section(&mut buf, &stage),

            _ => (),
        };
    }

    println!("{}", String::from_utf8(buf).unwrap());
}

/// temp
pub fn do_stuff() {
    do_thing_internal()
    // println!("{DEFAULT_FORMAT:?}");
}

struct StageInfo;
impl StageInfo {
    pub fn enemies_appearing(buf: &mut Vec<u8>, stage: &Stage) {
        buf.write(b"{{EnemiesAppearing").unwrap();
        for enemy in stage.enemies.iter() {
            if enemy.id == 21 {
                continue;
            }
            write!(buf, "|{}", enemy.id).unwrap();
        }
        buf.write(b"}}").unwrap();
    }

    pub fn intro(buf: &mut Vec<u8>, stage: &Stage, data: &StageWikiData) {
        write!(
            buf,
            "'''{name}''' is the {ordinal}{is_final} {stage_in} {map_name}.",
            name = data.stage_name.name,
            ordinal = get_ordinal(stage.meta.stage_num + 1),
            is_final = match data.stage_map.get(stage.meta.stage_num + 1) {
                None => " and final",
                _ => "",
            },
            stage_in = match stage.meta.type_enum {
                StageTypeEnum::Tower => "floor of",
                _ => "stage in",
            },
            map_name = data.stage_map.name,
            // TODO punctuation at end
        )
        .unwrap();

        if stage.is_no_continues {
            buf.write(b" This is a [[No Continues]] stage.").unwrap();
        }
    }

    pub fn restrictions_section(buf: &mut Vec<u8>, stage: &Stage) {
        buf.truncate(buf.len() - "\n\n==Restrictions==\n".len());
        let _ = stage;
    }
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
