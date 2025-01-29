use std::fmt::Write;

use crate::{
    config::Config,
    data::map::{
        map_data::{self, GameMap},
        parsed::map::{MapData, ResetType},
    },
    wikitext::{
        data_files::stage_wiki_data::{MapData as MapData2, STAGE_WIKI_DATA},
        format_parser::{parse_info_format, ParseType},
    },
};

const FORMAT: &str = "${map_img}
${intro}

==Difficulty==
?
${difficulties}

==List of Stages==
${stage_table}

${materials}

==Reference==
${ref}

----
${nav}
----

${footer}";

/// Ensure that expected stage invariants are met.
fn test_invariants(map: &MapData) {
    // assert_eq!(map.crown_data, None);
    assert_eq!(map.reset_type, ResetType::None);
    assert_eq!(map.max_clears, None);
    assert_eq!(map.cooldown, None);
    // assert_eq!(map.star_mask, None);
    assert_eq!(map.hidden_upon_clear, false);
    // assert_eq!(map.restrictions, None);
    assert_eq!(map.ex_option_map, None);
    assert_eq!(map.special_rule, None);
}

fn map_img(map: &MapData, _config: &Config) -> String {
    // TODO remove config
    let data = GameMap::new(&map.meta, _config.version.current_version());
    let map_num = data.map_file_num;
    format!("[[File:Map{map_num:03}.png|center|350px]]")
}

fn get_map_variable(name: &str, map: &MapData, map_data: &MapData2, _config: &Config) -> String {
    // TODO rename MapData2
    match name {
        "map_img" => map_img(map, _config),
        _ => format!("${{{name}}}"),
    }
}

pub fn get_legend_map(map: &MapData, config: &Config) -> String {
    test_invariants(map);

    // println!("{map:#?}");
    let map_data = STAGE_WIKI_DATA
        .stage_map(map.meta.type_num, map.meta.map_num)
        .unwrap_or_else(|| {
            panic!(
                "Couldn't find map name: {:03}-{:03}",
                map.meta.type_num, map.meta.map_num
            )
        });

    let mut buf = String::new();
    for node in parse_info_format(FORMAT) {
        if node.ptype == ParseType::Text {
            buf.write_str(node.content).unwrap();
            continue;
        }

        let new_buf = get_map_variable(node.content, map, map_data, config);
        buf.write_str(&new_buf).unwrap();
    }

    buf
}
