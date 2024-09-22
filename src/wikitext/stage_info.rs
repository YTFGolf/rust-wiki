//! Prints information about a stage.

use crate::wikitext::format_parser::parse_si_format;

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
${restriction}
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
${restrictions}

==Battleground==
${battlegrounds}

==Strategy==
-

==Reference==
*${reference}\
";

fn do_thing_internal() {
    let format = DEFAULT_FORMAT;
    let format_parsed = parse_si_format(&format);
    println!("{format_parsed:?}");
}

/// temp
pub fn do_stuff() {
    do_thing_internal()
    // println!("{DEFAULT_FORMAT:?}");
}
