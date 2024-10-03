//! Prints information about a stage.

mod internal;
use super::data_files::stage_page_data::{MapData, StageData, STAGE_NAMES};
use super::format_parser::{parse_si_format, ParseType};
use crate::data::stage::parsed::stage::Stage;
use regex::Regex;
use std::io::Write;

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
${score_rewards}
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
            "enemies_appearing" => internal::enemies_appearing(&stage),
            "intro" => internal::intro(&stage, &stage_wiki_data),
            "stage_name" => internal::stage_name(&stage).to_u8s(),
            "stage_location" => internal::stage_location(&stage).to_u8s(),
            "energy" => internal::energy(&stage)
                .map(|param| param.to_u8s())
                .unwrap_or(b"".to_vec()),
            "base_hp" => internal::base_hp(&stage)
                .into_iter()
                .map(|p| p.to_u8s())
                .collect::<Vec<Vec<u8>>>()
                .join(&b"\n"[..]),
            "enemies_list" => internal::enemies_list(&stage)
                .into_iter()
                .map(|p| p.to_u8s())
                .collect::<Vec<Vec<u8>>>()
                .join(b"\n".as_slice()),
            "treasure" => internal::treasure(&stage)
                .map(|param| param.to_string().as_bytes().to_vec())
                .unwrap_or(b"".to_vec()),
            "restrictions_info" => internal::restrictions_info(&stage)
                .map(|param| param.to_string().as_bytes().to_vec())
                .unwrap_or(b"".to_vec()),
            "score_rewards" => internal::score_rewards(&stage)
                .map(|param| param.to_string().as_bytes().to_vec())
                .unwrap_or(b"".to_vec()),
            "restrictions_section" => internal::restrictions_section(&stage).as_bytes().to_vec(),

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

// No context stage enemy line, just takes &enemy and show_mag. if base then can
// just write it in the function directly, and ** can be written there too.

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
        |enemy castle hp4 = 900,000 HP\
        "
        );
*/
