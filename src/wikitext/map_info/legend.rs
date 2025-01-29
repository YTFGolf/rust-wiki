use crate::{
    config::Config,
    data::{
        map::{
            map_data::GameMap,
            parsed::map::{MapData, ResetType},
        },
        stage::raw::stage_metadata::consts::StageTypeEnum,
        version::Version,
    },
    wikitext::{
        data_files::stage_wiki_data::{MapData as MapData2, STAGE_WIKI_DATA},
        format_parser::{parse_info_format, ParseType},
        wiki_utils::{extract_name, get_ordinal},
    },
};
use num_format::{Locale, WriteFormatted};
use std::fmt::Write;

const FORMAT: &str = "${map_img}
${intro}

==Difficulty==
${difficulty}

==List of Stages==
${stage_table}

${materials}

==Reference==
*${ref}

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

fn intro(map: &MapData, map_data: &MapData2, version: &Version) -> String {
    let mut buf = String::new();
    write!(
        buf,
        "'''{name}''' (?, ''?'', '''?''') is the {num} sub-chapter of {chap}, ",
        name = extract_name(&map_data.name),
        num = get_ordinal(map.meta.map_num + 1),
        chap = STAGE_WIKI_DATA.stage_type(map.meta.type_num).unwrap().name,
    )
    .unwrap();

    let map_offset = match map.meta.type_enum {
        StageTypeEnum::SoL => 0,
        StageTypeEnum::UL => 49,
        StageTypeEnum::ZL => 98,
        x => panic!("Type not compatible with Legend Stages: {x:?}."),
    };

    write!(
        buf,
        "and the {num} sub-chapter overall. ",
        num = get_ordinal(map.meta.map_num + 1 + map_offset)
    )
    .unwrap();

    let mut ver = version.number();
    if let Some(s) = ver.strip_suffix(".0") {
        ver = s
    }
    let ver = ver;

    write!(
        buf,
        "It was introduced in [[Version {ver} Update|Version {ver}]] \
        and is available up to {{{{{diff}c}}}} difficulty.",
        diff = map.crown_data.as_ref().unwrap().max_difficulty
    )
    .unwrap();
    buf
}

fn difficulty(map: &MapData) -> String {
    let data = map.crown_data.as_ref().unwrap();
    if u8::from(data.max_difficulty) == 1 {
        return String::new();
    }

    let mut buf = "{{LegendDiff".to_string();

    if let Some(mag) = data.crown_2 {
        buf.write_char('|').unwrap();
        buf.write_formatted(&mag, &Locale::en).unwrap();
    }
    if let Some(mag) = data.crown_3 {
        buf.write_char('|').unwrap();
        buf.write_formatted(&mag, &Locale::en).unwrap();
    }
    if let Some(mag) = data.crown_4 {
        buf.write_char('|').unwrap();
        buf.write_formatted(&mag, &Locale::en).unwrap();
    }

    buf.write_str("}}").unwrap();
    buf
}

fn reference(map: &MapData) -> String {
    let mapid = GameMap::get_map_id(&map.meta);
    format!("https://battlecats-db.com/stage/s{mapid:05}.html")
}

fn nav(map: &MapData, map_data: &MapData2) -> String {
    const BEGINNING: &str = "<p style=\"text-align:center;\">";
    const PREV: &str = "&lt;&lt;";
    const NEXT: &str = "&gt;&gt;";
    let mut buf = BEGINNING.to_string();

    let type_data = &STAGE_WIKI_DATA.stage_type(map.meta.type_num).unwrap();
    let chap = extract_name(&type_data.name);
    write!(buf, "[[:Category:{chap} Chapters|{chap} Chapters]]").unwrap();
    write!(buf, ":</p>\n\n{BEGINNING}'''").unwrap();

    let prev = match map.meta.map_num {
        0 => None,
        n => type_data.get(n - 1),
    };
    match prev {
        None => write!(buf, "{PREV} N/A").unwrap(),
        Some(data) => {
            let n = extract_name(&data.name);
            write!(buf, "[[{n}|{PREV} {n}]]").unwrap();
        }
    }
    buf.write_str(" | ").unwrap();
    match type_data.get(map.meta.map_num + 1) {
        None => write!(buf, "N/A {NEXT}").unwrap(),
        Some(data) => {
            let n = extract_name(&data.name);
            write!(buf, "[[{n}|{n} {NEXT}]]").unwrap();
        }
    }
    buf.write_str("'''</p>").unwrap();

    buf
}

fn footer(map: &MapData) -> String {
    match map.meta.type_enum {
        StageTypeEnum::SoL => {
            "{{LegendStages}}\n\
            [[Category:Stories of Legend Chapters]]"
        }
        StageTypeEnum::UL => {
            "{{UncannyLegendStages}}\n\
            [[Category:Uncanny Legends Chapters]]"
        }
        StageTypeEnum::ZL => {
            "{{ZeroLegendStages}}\n\
            [[Category:Zero Legends Chapters]]"
        }
        _ => unreachable!(),
    }
    .to_string()
}

fn get_map_variable(name: &str, map: &MapData, map_data: &MapData2, config: &Config) -> String {
    // TODO rename MapData2
    match name {
        "map_img" => map_img(map, config),
        "intro" => intro(map, map_data, &config.version.current_version()),
        "difficulty" => difficulty(map),
        // "stage_table"=>stage_table(map),
        // "materials"=>materials(map),
        "ref" => reference(map),
        "nav" => nav(map, map_data),
        "footer" => footer(map),
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
