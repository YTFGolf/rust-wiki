//! Beginning of stage info i.e. EnemiesAppearing and intro.

use crate::{
    data::stage::parsed::stage::Stage,
    meta::stage::variant::StageVariantID,
    wikitext::{
        data_files::enemy_data::ENEMY_DATA,
        stage_info::StageWikiData,
        wiki_utils::{extract_name, get_ordinal, OLD_OR_REMOVED_SUB},
    },
};
use std::{collections::HashSet, fmt::Write};

/// Get the enemies appearing line.
pub fn enemies_appearing(stage: &Stage) -> String {
    let mut buf = String::new();
    buf.write_str("{{EnemiesAppearing").unwrap();

    let mut displayed = HashSet::new();
    let enemies = stage
        .enemies
        .iter()
        .filter(|e| e.id != 21 && displayed.insert(e.id));

    for enemy in enemies {
        write!(buf, "|{}", ENEMY_DATA.get_common_name(enemy.id)).unwrap();
    }
    buf.write_str("}}").unwrap();

    buf
}

/// Get the "{stage} is the nth stage in {map}." line.
pub fn intro(stage: &Stage, data: &StageWikiData) -> String {
    if stage.id.variant() == StageVariantID::RankingDojo {
        return format!(
            "'''{extracted_name}''' is the {num} [[Arena of Honor]] of the [[Catclaw Dojo]].",
            extracted_name = extract_name(&data.stage_name.name),
            num = get_ordinal(stage.id.map().num() + 1)
        );
    }

    let mut buf = String::new();
    write!(
        buf,
        "'''{name}''' is the ",
        name = extract_name(&data.stage_name.name)
    )
    .unwrap();

    let num = stage.id.num();
    match (num, data.stage_map.get(num + 1)) {
        (0, None) => {
            buf.write_str("only").unwrap();
        }
        (n, None) => {
            write!(buf, "{ord} and final", ord = get_ordinal(n + 1)).unwrap();
        }
        (n, Some(_)) => {
            write!(buf, "{ord}", ord = get_ordinal(n + 1)).unwrap();
        }
    };
    // only/nth/nth and final

    write!(
        buf,
        " {stage_in} {map_name}{punct}",
        stage_in = match stage.id.variant() {
            StageVariantID::Tower => "floor of",
            _ => "stage in",
        },
        map_name = OLD_OR_REMOVED_SUB.replace(&data.stage_map.name, "$1"),
        punct = match extract_name(&data.stage_map.name).chars().last().unwrap() {
            '!' | '.' => "",
            _ => ".",
        }
    )
    .unwrap();

    if stage.is_no_continues {
        buf.write_str(" This is a [[No Continues]] stage.").unwrap();
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::super::test_util::get_stage_wiki_data;
    use super::*;

    #[test]
    fn test_enemies_appearing() {
        let crazed_cat = Stage::new_current("s 17 0").unwrap();
        let buf = enemies_appearing(&crazed_cat);
        assert_eq!(
            &buf,
            "{{EnemiesAppearing|Le'boin|Teacher Bear|Doge|Snache|Croco|Crazed Cat}}"
        );
        // normal

        let tada = Stage::new_current("ex 63 0").unwrap();
        let buf = enemies_appearing(&tada);
        assert_eq!(&buf, "{{EnemiesAppearing}}");
        // blank

        let not_alone = Stage::new_current("c 176 4").unwrap();
        let buf = enemies_appearing(&not_alone);
        assert_eq!(&buf, "{{EnemiesAppearing|Shibalien|Mistress Celeboodle|Imperator Sael|Kroxo|Cyberhorn|Dessert Witch - Obsession}}");
        // used to be charlotte but that changed in 14.1 so kinda pointless now
        // but then again no reason to remove it

        let star_ocean = Stage::new_current("sol 15 7").unwrap();
        let buf = enemies_appearing(&star_ocean);
        assert_eq!(
            &buf,
            "{{EnemiesAppearing|Doge|Those Guys|Doge Dark|H. Nah}}"
        );
        // doge dark has multiple entries in enemies_list
    }

    #[test]
    fn test_intro() {
        let ht30 = Stage::new_current("v 0 29").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&ht30);
        let buf = intro(&ht30, &stage_wiki_data);
        assert_eq!(&buf, "'''Floor 30''' is the 30th floor of [[Heavenly Tower]]. This is a [[No Continues]] stage.");
        // tower; no continues

        let whole_new = Stage::new_current("zl 0 0").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&whole_new);
        let buf = intro(&whole_new, &stage_wiki_data);
        assert_eq!(&buf, "'''A Whole New World''' is the only stage in [[Zero Field]]. This is a [[No Continues]] stage.");
        // only

        let earthshaker = Stage::new_current("sol 0 0").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&earthshaker);
        let buf = intro(&earthshaker, &stage_wiki_data);
        assert_eq!(
            &buf,
            "'''Earthshaker''' is the first stage in [[The Legend Begins]]."
        );
        // normal stage

        let refusal_type = Stage::new_current("c 206 1").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&refusal_type);
        let buf = intro(&refusal_type, &stage_wiki_data);
        assert_eq!(&buf, "'''Refusal Type (Merciless)''' is the second and final stage in [[The 10th Angel Strikes!]] This is a [[No Continues]] stage.");
        // ! in map name; final

        let crimson_trial = Stage::new_current("r 20 0").unwrap();
        let stage_wiki_data = get_stage_wiki_data(&crimson_trial);
        let buf = intro(&crimson_trial, &stage_wiki_data);
        assert_eq!(
            &buf,
            "'''Crimson Trial''' is the 21st [[Arena of Honor]] of the [[Catclaw Dojo]]."
        );
        // arena of honor
    }
}
