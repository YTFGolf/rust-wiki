//! Common functions for map info sub-scripts.

use crate::{
    game_data::{
        map::{parsed::map::GameMap, raw::map_data::GameMapData},
        meta::stage::{stage_id::StageID, stage_types::transform::transform_map::map_img_code},
        version::Version,
    },
    interface::error_handler::InfallibleWrite,
    wiki_data::stage_wiki_data::MapWikiData,
    wikitext::text_utils::extract_link,
};
use num_format::{Locale, ToFormattedString};
use std::fmt::Write;

/// Table showing what stages are available in the map.
pub fn stage_table(map_data: &GameMap, map_wiki_data: &MapWikiData, version: &Version) -> String {
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

        // link=link|name at end of image
        let link_part = {
            let stagelink = extract_link(&stage.name);
            let mut l = format!("link={stagelink}");

            let stagename = extract_link(&stage.name);
            if stagename != stagelink {
                write!(l, "|{stagename}").infallible_write();
            }

            l
        };

        write!(
            buf,
            "\n|-\n\
            ! scope=\"row\" | Stage {stagenum2}\n\
            | [[File:Mapsn{mapnum:03} {stagenum:02} {code} en.png|200px|{link_part}]]\n\
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
        )
        .unwrap();

        i += 1;
    }
    buf.write_str("\n|}").infallible_write();

    buf
}
