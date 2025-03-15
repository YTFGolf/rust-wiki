use crate::{
    config::Config,
    data::{
        map::{
            map_data::GameMapData,
            parsed::map::{GameMap, ResetType},
        },
        version::Version,
    },
    meta::stage::{
        map_id::MapID, stage_id::StageID, stage_types::transform::transform_map::map_img_code,
        variant::StageVariantID,
    },
    wikitext::{
        data_files::stage_wiki_data::{MapWikiData, STAGE_WIKI_DATA},
        format_parser::{ParseType, parse_info_format},
        wiki_utils::{extract_link, extract_name, get_ordinal},
    },
};
use num_format::{Locale, ToFormattedString, WriteFormatted};
use std::fmt::Write;

/// Base [format][parse_info_format] string for Legend Stages.
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

/// Subset of [`StageVariantID`] available in Legend Stages.
enum LegendSubset {
    SoL,
    UL,
    ZL,
}
impl From<StageVariantID> for LegendSubset {
    fn from(value: StageVariantID) -> Self {
        match value {
            StageVariantID::SoL => Self::SoL,
            StageVariantID::UL => Self::UL,
            StageVariantID::ZL => Self::ZL,
            x => panic!("Type not compatible with Legend Stages: {x:?}."),
        }
    }
}

/// Ensure that expected stage invariants are met.
fn test_invariants(map: &GameMap) {
    // assert_eq!(map.crown_data, None);
    assert_eq!(map.reset_type, ResetType::None);
    assert_eq!(map.max_clears, None);
    assert_eq!(map.cooldown, None);
    // assert_eq!(map.star_mask, None);
    assert!(!map.hidden_upon_clear);
    // assert_eq!(map.restrictions, None);
    assert_eq!(map.ex_option_map, None);
    assert_eq!(map.special_rule, None);
}

/// Map's background image.
fn map_img(map: &GameMap) -> String {
    format!("[[File:Map{:03}.png|center|350px]]", map.map_file_num)
}

/// Introduction sentences.
fn intro(map: &GameMap, map_data: &MapWikiData, config: &Config) -> String {
    let mut buf = String::new();
    write!(
        buf,
        "'''{name}''' (?, ''?'', '''?''') is the {num} sub-chapter of {chap}",
        name = extract_name(&map_data.name),
        num = get_ordinal(map.id.num() + 1),
        chap = STAGE_WIKI_DATA.stage_type(map.id.variant()).unwrap().name,
    )
    .unwrap();

    let map_offset = match map.id.variant().into() {
        LegendSubset::SoL => 0,
        LegendSubset::UL => 49,
        LegendSubset::ZL => 98,
    };

    if map_offset != 0 {
        write!(
            buf,
            ", and the {num} sub-chapter overall",
            num = get_ordinal(map.id.num() + 1 + map_offset)
        )
        .unwrap();
    }
    buf += ". ";

    buf.write_str("It ").unwrap();
    if config.map_info.version() {
        let mut ver = config.version.current_version().number();
        if let Some(s) = ver.strip_suffix(".0") {
            ver = s;
        }

        write!(
            buf,
            "was introduced in [[Version {ver} Update|Version {ver}]] and "
        )
        .unwrap();
    }
    write!(
        buf,
        "is available up to {{{{{diff}c}}}} difficulty.",
        diff = map.crown_data.as_ref().unwrap().max_difficulty
    )
    .unwrap();
    buf
}

/// LegendDiff invocation for map.
fn difficulty(map: &GameMap) -> String {
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

/// Table showing what stages are available in the map.
fn stage_table(map_data: &GameMap, map_wiki_data: &MapWikiData, version: &Version) -> String {
    let mapnum = map_data.id.num();
    let code = map_img_code(&map_data.id);

    let mut buf = format!(
        "{{| class=\"article-table\"\n\
        ! rowspan=\"2\" scope=\"row\" |\n\
        ! rowspan=\"2\" scope=\"col\" | [[File:Mapname{mapnum:03} {code} en.png|200px]]\n\
        ! rowspan=\"2\" scope=\"col\" | [[File:Mapname{mapnum:03} {code} ja.png|200px]]\n\
        ! colspan=\"2\" scope=\"col\" style=\"text-align: center; min-width: 210px; \
        line-height: 20px;\" |'''Difficulty'''<br>{{{{Stars|{star_mask}}}}}\n\
        |-\n\
        ! scope=\"col\" | Translation\n\
        ! scope=\"col\" | Energy",
        star_mask = map_data.star_mask.unwrap_or_default()
    );

    let mut i = 0;
    while let Some(stage) = map_wiki_data.get(i) {
        let stage_id = StageID::from_map(map_data.id.clone(), i);
        write!(
            buf,
            "\n|-\n\
            ! scope=\"row\" | Stage {stagenum2}\n\
            | [[File:Mapsn{mapnum:03} {stagenum:02} {code} en.png|200px|link={stagename}]]\n\
            | [[File:Mapsn{mapnum:03} {stagenum:02} {code} ja.png|200px|?]]\n\
            | ?\n\
            | {energy} {{{{EnergyIcon}}}}",
            stagenum = i,
            stagenum2 = i + 1,
            // TODO this really shouldn't be dealing with `GameMapData`
            energy = GameMapData::get_stage_data(&stage_id, version)
                .unwrap()
                .fixed_data
                .energy
                .to_formatted_string(&Locale::en),
            stagename = extract_link(&stage.name)
        )
        .unwrap();

        i += 1;
    }
    buf.write_str("\n|}").unwrap();

    buf
}

/// Stage's material drops.
fn materials(map_data: &GameMap, version: &Version) -> String {
    fn format_material(miss_chance: u8, chances: &str) -> String {
        format!("{{{{Materials|{miss_chance}{chances}}}}}")
    }

    let drop_item = GameMapData::get_drop_item(&map_data.id, version).unwrap();
    let normal = [
        drop_item.bricks,
        drop_item.feathers,
        drop_item.coal,
        drop_item.sprockets,
        drop_item.gold,
        drop_item.meteorite,
        drop_item.beast_bones,
        drop_item.ammonite,
    ];

    let mut total = 0;
    // TODO miss chance is definitely wrong
    let mut buf = String::new();

    for chance in normal {
        write!(buf, "|{chance}").unwrap();
        total += chance;
    }
    if drop_item.brick_z.is_none() {
        return format_material(100 - total, &buf);
    }

    let drops_z = [
        drop_item.brick_z,
        drop_item.feathers_z,
        drop_item.coal_z,
        drop_item.sprockets_z,
        drop_item.gold_z,
        drop_item.meteorite_z,
        drop_item.beast_bones_z,
        drop_item.ammonite_z,
    ];
    for chance in drops_z {
        let chance = chance.unwrap();
        write!(buf, "|{chance}").unwrap();
        total += chance;
    }
    buf.write_str("|hidenormal=").unwrap();

    format_material(100 - total, &buf)
}

/// battlecats-db reference.
fn reference(map: &GameMap) -> String {
    let mapid = map.id.mapid();
    format!("https://battlecats-db.com/stage/s{mapid:05}.html")
}

/// Format navigation.
// TODO this should probably be extracted out somewhere.
fn nav_item(heading: &str, left: &str, right: &str) -> String {
    const START: &str = "<p style=\"text-align:center;\">";
    const END: &str = "</p>";

    format!(
        "{START}{heading}:{END}\n\n\
        {START}'''{left} | {right}'''{END}"
    )
}

/// Format navigation.
// TODO this should also probably be extracted out somewhere.
fn nav_item_opt(heading: &str, left: Option<&str>, right: Option<&str>) -> String {
    const PREV: &str = "&lt;&lt;";
    const NEXT: &str = "&gt;&gt;";

    let left = match left {
        None => format!("{PREV} N/A"),
        Some(name) => {
            format!("[[{name}|{PREV} {name}]]")
        }
    };
    let right = match right {
        None => format!("N/A {NEXT}"),
        Some(name) => {
            format!("[[{name}|{name} {NEXT}]]")
        }
    };

    nav_item(heading, &left, &right)
}

/// Navigation menu for map.
fn nav(map: &GameMap) -> String {
    let type_data = &STAGE_WIKI_DATA.stage_type(map.id.variant()).unwrap();
    let chap = extract_name(&type_data.name);
    let heading = format!("[[:Category:{chap} Chapters|{chap} Chapters]]");

    let prev = match map.id.num() {
        0 => None,
        n => type_data.get(n - 1),
    };
    let left = prev.map(|data| extract_name(&data.name));
    let right = type_data
        .get(map.id.num() + 1)
        .map(|data| extract_name(&data.name));

    nav_item_opt(&heading, left, right)
}

/// Footer (templates/categories).
fn footer(map: &GameMap) -> String {
    match map.id.variant().into() {
        LegendSubset::SoL => {
            "{{LegendStages}}\n\
            [[Category:Stories of Legend Chapters]]"
        }
        LegendSubset::UL => {
            "{{UncannyLegendStages}}\n\
            [[Category:Uncanny Legends Chapters]]"
        }
        LegendSubset::ZL => {
            "{{ZeroLegendStages}}\n\
            [[Category:Zero Legends Chapters]]"
        }
    }
    .to_string()
}

/// Get variable defined in format.
fn get_map_variable(name: &str, map: &GameMap, map_data: &MapWikiData, config: &Config) -> String {
    let version = &config.version.current_version();
    match name {
        "map_img" => map_img(map),
        "intro" => intro(map, map_data, config),
        "difficulty" => difficulty(map),
        "stage_table" => stage_table(map, map_data, version),
        "materials" => materials(map, version),
        "ref" => reference(map),
        "nav" => nav(map),
        "footer" => footer(map),
        invalid => panic!("Variable {invalid:?} is not recognised!"),
    }
}

/// Get map's wiki data.
fn get_map_wiki_data(map: &MapID) -> &'static MapWikiData {
    STAGE_WIKI_DATA.stage_map(map).unwrap_or_else(|| {
        panic!(
            "Couldn't find map name: {:03}-{:03}",
            map.variant().num(),
            map.num()
        )
    })
}

/// Get map data for legend stages.
pub fn get_legend_map(map: &GameMap, config: &Config) -> String {
    test_invariants(map);

    // println!("{map:#?}");
    let map_wiki_data = get_map_wiki_data(&map.id);

    let mut buf = String::new();
    for node in parse_info_format(FORMAT) {
        if node.ptype == ParseType::Text {
            buf.write_str(node.content).unwrap();
            continue;
        }

        let new_buf = get_map_variable(node.content, map, map_wiki_data, config);
        buf.write_str(&new_buf).unwrap();
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::TEST_CONFIG, meta::stage::map_id::MapID};

    #[test]
    fn test_full() {
        let mut config = TEST_CONFIG.clone();
        config.map_info.set_version(false);
        config.version.init_all();
        let version = config.version.current_version();

        let leg_begins = GameMap::from_id(MapID::from_numbers(0, 0), version);
        let map_data = get_map_wiki_data(&leg_begins.id);

        assert_eq!(map_img(&leg_begins), "[[File:Map004.png|center|350px]]");
        assert_eq!(
            intro(&leg_begins, map_data, &config),
            "'''The Legend Begins''' (?, ''?'', '''?''') is the first sub-chapter of \
            [[Legend Stages#Stories of Legend|Stories of Legend]]. \
            It is available up to {{4c}} difficulty."
        );
        assert_eq!(difficulty(&leg_begins), "{{LegendDiff|150|200|300}}");
        // assert_eq!(stage_table(&leg_begins, map_data, version), "");
        assert_eq!(
            materials(&leg_begins, version),
            "{{Materials|61|13|0|13|13|0|0|0|0}}"
        );
        assert_eq!(
            reference(&leg_begins),
            "https://battlecats-db.com/stage/s00000.html"
        );
        assert_eq!(
            nav(&leg_begins),
            nav_item(
                "[[:Category:Stories of Legend Chapters|Stories of Legend Chapters]]",
                "&lt;&lt; N/A",
                "[[Passion Land|Passion Land &gt;&gt;]]"
            )
        );
        assert_eq!(
            footer(&leg_begins),
            "{{LegendStages}}\n[[Category:Stories of Legend Chapters]]"
        );

        assert_eq!(
            get_legend_map(&leg_begins, &config),
            include_str!("leg_begins.txt").trim()
        );
    }

    #[test]
    fn test_version() {
        let mut with_version = TEST_CONFIG.clone();
        with_version.map_info.set_version(true);
        with_version.version.init_all();
        let version = with_version.version.current_version();

        let leg_begins = GameMap::from_id(MapID::from_numbers(0, 0), version);
        let map_data = get_map_wiki_data(&leg_begins.id);

        let mut ver = version.number();
        if let Some(s) = ver.strip_suffix(".0") {
            ver = s;
        }
        let target = format!(
            "'''The Legend Begins''' (?, ''?'', '''?''') is the first sub-chapter of \
            [[Legend Stages#Stories of Legend|Stories of Legend]]. It \
            was introduced in [[Version {ver} Update|Version {ver}]] and \
            is available up to {{{{4c}}}} difficulty."
        );
        assert_eq!(intro(&leg_begins, map_data, &with_version), target);

        let mut no_version = with_version;
        no_version.map_info.set_version(false);
        assert_eq!(
            intro(&leg_begins, map_data, &no_version),
            "'''The Legend Begins''' (?, ''?'', '''?''') is the first sub-chapter of \
            [[Legend Stages#Stories of Legend|Stories of Legend]]. \
            It is available up to {{4c}} difficulty."
        );
    }
}
