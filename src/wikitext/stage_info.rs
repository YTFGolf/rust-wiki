//! Prints information about a stage.

use crate::{
    data::stage::parsed::stage::Stage,
    wikitext::format_parser::{parse_si_format, ParseType},
};
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

fn do_thing_internal() {
    let format = DEFAULT_FORMAT;
    let parsed = parse_si_format(&format);

    let mut buf = vec![];
    let stage = Stage::new("n 0 0").unwrap();

    for node in parsed {
        if node.ptype == ParseType::Text {
            buf.write(node.content.as_bytes()).unwrap();
            continue;
        }

        match node.content {
            "enemies_appearing" => StageInfo::enemies_appearing(&mut buf, &stage),
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

    pub fn restrictions_section(buf: &mut Vec<u8>, stage: &Stage) {
        buf.truncate(buf.len() - "\n\n==Restrictions==\n".len());
    }
}
