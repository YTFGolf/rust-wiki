//! Module for testing stage info.
#![cfg(test)]

use super::{get_stage_wiki_data, variables::get_stage_variable};
use crate::{
    config::{Config, TEST_CONFIG, version_config::Lang},
    game_data::stage::parsed::stage::Stage,
    meta::stage::{stage_id::StageID, variant::StageVariantID as T},
};

// these were all generated on a different branch.

pub const EARTHSHAKER: &str = "{{Stage Info
|stage name = [[File:rc000.png]]
[[File:Mapsn000 00 n en.png]]
|stage location = [[File:Mapname000 n en.png]]
|energy = 50
|enemy castle hp = 60,000 HP
|enemies = {{Magnification|Doge|200%
|Snache|200%
|Those Guys|200%}}
|enemies2 = {{Magnification|Doge|300%
|Snache|300%
|Those Guys|300%}}
|enemies3 = {{Magnification|Doge|400%
|Snache|400%
|Those Guys|400%}}
|enemies4 = {{Magnification|Doge|600%
|Snache|600%
|Those Guys|600%}}
|treasure = - [[Battle Items#Speed Up|Speed Up]] +1 (1%, unlimited)
|XP = 950 XP
|width = 4,200
|max enemies = 7
|jpname = ?
|script = ?
|romaji = ?
|star = 4
|sub-chapter = [[The Legend Begins]]
|difficulty = ★1
|prev stage = N/A
|next stage = [[Return of Terror]]
}}";

pub const FINALE:&str="{{Stage Info
|stage name = [[File:E 651.png]]
[[File:Mapsn209 00 c en.png]]
|stage location = [[File:Mapname209 c en.png]]
|energy = 30
|enemy castle hp = 50 HP
|base = {{Magnification|Finale Base|100%}}
|treasure = - [[Cat Capsule#Rare Cat Capsule|Rare Ticket]] +1 (100%, 1 time)
|restriction = Unit Restriction: Only [[Cat (Normal Cat)|Cat]]
|XP = 2,700 XP
|width = 4,500
|max enemies = 20
|jpname = ?
|script = ?
|romaji = ?
|star = 1
|event = [[Neon Genesis Evangelion Collaboration Event]]
|event-chapter = [[Neon Genesis Evangelion Collaboration Event#Bye-bye, all of the Cats|Bye-bye, all of the Cats]]
|difficulty = ★1
|prev stage = N/A
|next stage = N/A
}}";

pub const SEAL_MAGS: &str = "{{Stage Info
|stage name = [[File:rc136.png]]
[[File:Mapsn000 19 a en.png]]
|stage location = [[File:Mapname000 a en.png]]
|energy = 100
|enemy castle hp = 600,000 HP
|enemies = {{Magnification|Doge|1,800%
|One Horn|1,800%
|Sir Seal|2,500%
|B.B.Bunny|1,800%
|Gory|3,000%}}
|boss = {{Magnification|Baron Seal|5,500%}}
|treasure = - 1,000,000 XP (100%, 1 time)
|restriction = [[No Continues]]
|XP = 1,900 XP
|width = 4,000
|max enemies = 12
|jpname = ?
|script = ?
|romaji = ?
|star = 1
|event-chapter = [[Baron Seal Strikes (Old)/Red Alert 1~20|Baron Seal Strikes!]]
|max clears = 1
|prev stage = [[Baron Seal Strikes (Old)/Red Alert 1~20|Red Alert 19]]
|next stage = N/A
}}";

pub const SEAL_NOMAG: &str = "{{Stage Info
|stage name = [[File:rc136.png]]
[[File:Mapsn000 19 a en.png]]
|stage location = [[File:Mapname000 a en.png]]
|energy = 100
|enemy castle hp = 600,000 HP
|enemies = {{Magnification|Doge|0
|One Horn|0
|Sir Seal|0
|B.B.Bunny|0
|Gory|0}}
|boss = {{Magnification|Baron Seal|0}}
|treasure = - 1,000,000 XP (100%, 1 time)
|restriction = [[No Continues]]
|XP = 1,900 XP
|width = 4,000
|max enemies = 12
|jpname = ?
|script = ?
|romaji = ?
|star = 1
|event-chapter = [[Baron Seal Strikes (Old)/Red Alert 1~20|Baron Seal Strikes!]]
|max clears = 1
|prev stage = [[Baron Seal Strikes (Old)/Red Alert 1~20|Red Alert 19]]
|next stage = N/A
}}";

pub const DOJO: &str = "{{Stage Info
|stage name = [[File:E 283.png]]
[[File:Mapsn000 00 t en.png]]
|stage location = [[File:Mapname000 t en.png]]
|energy = 0
|enemy castle hp = Unlimited
|base = {{Magnification|Scarecrow|0}}
|enemies = {{Magnification|One Horn|0
|Doge Dark|0
|St. Pigge the 2nd|0
|Squire Rel|0
|R.Ost|0
|Shadow Boxer K|0
|Dagshund|0
|Le'boin|0}}
|boss = {{Magnification|The Face|0
|Squire Rel|0
|R.Ost|0
|St. Pigge the 2nd|0
|Le'boin|0}}
|XP = 0 XP
|width = 4,200
|max enemies = 12
|jpname = ?
|script = ?
|romaji = ?
|star = 1
|dojo-chapter = [[Catclaw Dojo|Hall of Initiates]]
|difficulty = ★2
}}";

fn get_config() -> Config {
    let mut config = TEST_CONFIG.clone();
    config.version.init_all();
    config.version.set_lang(Lang::EN);
    config
}

#[test]
fn info_earthshaker() {
    let earthshaker = StageID::from_components(T::SoL, 0, 0);
    let wik = get_stage_wiki_data(&earthshaker);
    let stage = get_stage_variable(
        "si_template",
        &Stage::from_id_current(earthshaker).unwrap(),
        &wik,
        &get_config(),
    );
    assert_eq!(stage, EARTHSHAKER);
}

#[test]
fn info_finale() {
    let finale = StageID::from_components(T::Collab, 209, 0);
    let wik = get_stage_wiki_data(&finale);
    let stage = get_stage_variable(
        "si_template",
        &Stage::from_id_current(finale).unwrap(),
        &wik,
        &get_config(),
    );
    assert_eq!(stage, FINALE);
}

#[test]
fn info_baron_mags() {
    let baron = StageID::from_components(T::Gauntlet, 0, 19);
    let wik = get_stage_wiki_data(&baron);

    let mut config = get_config();
    config.stage_info.set_suppress(false);

    let stage = get_stage_variable(
        "si_template",
        &Stage::from_id_current(baron).unwrap(),
        &wik,
        &config,
    );
    assert_eq!(stage, SEAL_MAGS);
}

#[test]
fn info_baron_nomags() {
    let baron = StageID::from_components(T::Gauntlet, 0, 19);
    let wik = get_stage_wiki_data(&baron);

    let mut config = get_config();
    config.stage_info.set_suppress(true);

    let stage = get_stage_variable(
        "si_template",
        &Stage::from_id_current(baron).unwrap(),
        &wik,
        &config,
    );
    assert_eq!(stage, SEAL_NOMAG);
}

#[test]
fn info_dojo() {
    let dojo = StageID::from_components(T::Dojo, 0, 0);
    let wik = get_stage_wiki_data(&dojo);
    let stage = get_stage_variable(
        "si_template",
        &Stage::from_id_current(dojo).unwrap(),
        &wik,
        &get_config(),
    );
    assert_eq!(stage, DOJO);
}
