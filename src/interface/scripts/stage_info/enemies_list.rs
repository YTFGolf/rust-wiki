//! Module for the [enemies_list] function.

use crate::{
    data::stage::parsed::{
        stage::Stage,
        stage_enemy::{BossType, MS_SIGN, StageEnemy},
    },
    wiki_data::enemy_data::ENEMY_DATA,
    interface::error_handler::InfallibleWrite,
    meta::stage::variant::StageVariantID as T,
    wikitext::template::TemplateParameter,
};
use either::Either::{Left, Right};
use num_format::{Locale, WriteFormatted};
use std::{collections::HashSet, fmt::Write};

/// Get list of enemies and their magnifications.
pub fn enemies_list(
    stage: &Stage,
    suppress_gauntlet_magnification: bool,
) -> Vec<TemplateParameter> {
    struct EnemyListWithDupes<'a> {
        base: Vec<&'a StageEnemy>,
        enemies: Vec<&'a StageEnemy>,
        boss: Vec<&'a StageEnemy>,
    }
    let mut enemy_list = EnemyListWithDupes {
        base: vec![],
        enemies: vec![],
        boss: vec![],
    };
    for enemy in &stage.enemies {
        if enemy.is_base {
            enemy_list.base.push(enemy);
        } else if enemy.boss_type == BossType::None {
            enemy_list.enemies.push(enemy);
        } else {
            enemy_list.boss.push(enemy);
        }
    }
    // get all enemies

    let suppress_magnification: bool = matches!(stage.id.variant(), T::Dojo | T::RankingDojo)
        || suppress_gauntlet_magnification && stage.id.variant().is_gauntlet();

    assert!(
        enemy_list.base.len() <= 1,
        "Stage has multiple enemy bases!"
    );
    let mut enemy_list_seen = HashSet::new();
    let mag_filter = match suppress_magnification {
        true => |_| Left(0),
        false => |mag| mag,
    };
    let filtered_enemies = enemy_list
        .enemies
        .into_iter()
        .filter(|e| e.id != MS_SIGN && enemy_list_seen.insert((e.id, mag_filter(e.magnification))))
        .collect::<Vec<&StageEnemy>>();
    let mut boss_list_seen = HashSet::new();
    let filtered_boss = enemy_list
        .boss
        .into_iter()
        .filter(|e| e.id != MS_SIGN && boss_list_seen.insert((e.id, mag_filter(e.magnification))))
        .collect::<Vec<&StageEnemy>>();
    // remove duplicates

    /// Write `|{enemy}|{mag}%` to `buf`. Multiplier is raw % i.e. 100 = *1.
    fn write_enemy(buf: &mut String, enemy: &StageEnemy, multiplier: u32) {
        write!(buf, "|{}|", ENEMY_DATA.get_common_name(enemy.id)).unwrap();
        match &enemy.magnification {
            Left(m) => {
                buf.write_formatted(&(m * multiplier / 100), &Locale::en)
                    .unwrap();
                buf.write_str("%").infallible_write();
            }
            Right((hp, ap)) => {
                buf.write_formatted(&(hp * multiplier / 100), &Locale::en)
                    .unwrap();
                buf.write_str("/").infallible_write();
                buf.write_formatted(&(ap * multiplier / 100), &Locale::en)
                    .unwrap();
                buf.write_str("%").infallible_write();
            }
        };
    }
    /// Write `|{enemy}|0` to `buf`.
    fn write_enemy_0(buf: &mut String, enemy: &StageEnemy, _: u32) {
        write!(buf, "|{}|0", ENEMY_DATA.get_common_name(enemy.id)).unwrap();
    }

    let write_enemy_f = match suppress_magnification {
        true => write_enemy_0,
        false => write_enemy,
    };
    let collect_all_enemies = |filtered_enemies_vec: &[&StageEnemy], multiplier: u32| {
        filtered_enemies_vec
            .iter()
            .map(|e| {
                let mut buf = String::new();
                write_enemy_f(&mut buf, e, multiplier);
                buf
            })
            .collect::<Vec<String>>()
            .join("\n")
    };
    // util functions

    let mut param_vec: Vec<TemplateParameter> = vec![];
    let mut add_to_enemy_vec = |key: &'static str, list: String| {
        let mut buf = String::new();
        buf.write_str("{{Magnification").infallible_write();
        buf.write_str(&list).infallible_write();
        buf.write_str("}}").infallible_write();

        param_vec.push(TemplateParameter::new(key, buf));
    };
    // return value and another util function (has to be a mutable closure
    // since it uses `enemy_vec`).

    if !enemy_list.base.is_empty() {
        let base_items = collect_all_enemies(&enemy_list.base, 100);
        add_to_enemy_vec("base", base_items);
    }
    if !filtered_enemies.is_empty() {
        let enemy_items = collect_all_enemies(&filtered_enemies, 100);
        add_to_enemy_vec("enemies", enemy_items);
    }
    if !filtered_boss.is_empty() {
        let boss_items = collect_all_enemies(&filtered_boss, 100);
        add_to_enemy_vec("boss", boss_items);
    }

    let crowns = match &stage.crown_data {
        None => return param_vec,
        Some(c) => c,
    };
    let difficulty: u8 = crowns.max_difficulty.into();
    if difficulty == 1 {
        return param_vec;
    }

    let magnif_2: u32 = crowns.crown_2.unwrap().into();
    if !enemy_list.base.is_empty() {
        let base_items = collect_all_enemies(&enemy_list.base, magnif_2);
        add_to_enemy_vec("base2", base_items);
    }
    if !filtered_enemies.is_empty() {
        let enemy_items = collect_all_enemies(&filtered_enemies, magnif_2);
        add_to_enemy_vec("enemies2", enemy_items);
    }
    if !filtered_boss.is_empty() {
        let boss_items = collect_all_enemies(&filtered_boss, magnif_2);
        add_to_enemy_vec("boss2", boss_items);
    }
    if difficulty == 2 {
        return param_vec;
    }

    let magnif_3: u32 = crowns.crown_3.unwrap().into();
    if !enemy_list.base.is_empty() {
        let base_items = collect_all_enemies(&enemy_list.base, magnif_3);
        add_to_enemy_vec("base3", base_items);
    }
    if !filtered_enemies.is_empty() {
        let enemy_items = collect_all_enemies(&filtered_enemies, magnif_3);
        add_to_enemy_vec("enemies3", enemy_items);
    }
    if !filtered_boss.is_empty() {
        let boss_items = collect_all_enemies(&filtered_boss, magnif_3);
        add_to_enemy_vec("boss3", boss_items);
    }
    if difficulty == 3 {
        return param_vec;
    }

    let magnif_4: u32 = crowns.crown_4.unwrap().into();
    if magnif_4 == 100 {
        return param_vec;
    }
    if !enemy_list.base.is_empty() {
        let base_items = collect_all_enemies(&enemy_list.base, magnif_4);
        add_to_enemy_vec("base4", base_items);
    }
    if !filtered_enemies.is_empty() {
        let enemy_items = collect_all_enemies(&filtered_enemies, magnif_4);
        add_to_enemy_vec("enemies4", enemy_items);
    }
    if !filtered_boss.is_empty() {
        let boss_items = collect_all_enemies(&filtered_boss, magnif_4);
        add_to_enemy_vec("boss4", boss_items);
    }

    param_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::stage::stage_id::StageID;

    #[test]
    fn simple_case() {
        let aac = Stage::from_id_current(StageID::from_components(T::UL, 0, 0)).unwrap();
        assert_eq!(
            enemies_list(&aac, true),
            vec![
                TemplateParameter::new("enemies", "{{Magnification|Relic Doge|100%}}"),
                TemplateParameter::new("boss", "{{Magnification|Relic Bun-Bun|100%}}"),
                TemplateParameter::new("enemies2", "{{Magnification|Relic Doge|150%}}"),
                TemplateParameter::new("boss2", "{{Magnification|Relic Bun-Bun|150%}}"),
                TemplateParameter::new("enemies3", "{{Magnification|Relic Doge|200%}}"),
                TemplateParameter::new("boss3", "{{Magnification|Relic Bun-Bun|200%}}"),
            ]
        );
    }

    #[test]
    fn blank_enemy_list() {
        let tada = Stage::from_id_current(StageID::from_components(T::Extra, 63, 0)).unwrap();
        assert_eq!(enemies_list(&tada, true), vec![]);
    }

    #[test]
    fn repeat_and_floating_error() {
        // i.e. Gabriel appears at different magnifications *and* it houses the
        // infamous 700% magnification with a 1.4x multiplier on 3-star, which
        // on Python always used to evaluate as 979%.
        let celestial_seas =
            Stage::from_id_current(StageID::from_components(T::SoL, 32, 3)).unwrap();
        assert_eq!(
            enemies_list(&celestial_seas, true),
            vec![
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|Doge|3,000%\n\
                    |Those Guys|2,000%\n\
                    |Gabriel|400%\n\
                    |Gabriel|600%\n\
                    |Gabriel|700%\n\
                    |Gabriel|800%\n\
                    |Gabriel|900%\n\
                    |Gabriel|1,000%\n\
                    |Gabriel|2,000%}}"
                ),
                TemplateParameter::new("boss", "{{Magnification|Le'boin|10,000%}}"),
                TemplateParameter::new(
                    "enemies2",
                    "{{Magnification|Doge|3,600%\n\
                    |Those Guys|2,400%\n\
                    |Gabriel|480%\n\
                    |Gabriel|720%\n\
                    |Gabriel|840%\n\
                    |Gabriel|960%\n\
                    |Gabriel|1,080%\n\
                    |Gabriel|1,200%\n\
                    |Gabriel|2,400%}}"
                ),
                TemplateParameter::new("boss2", "{{Magnification|Le'boin|12,000%}}"),
                TemplateParameter::new(
                    "enemies3",
                    "{{Magnification|Doge|4,200%\n\
                    |Those Guys|2,800%\n\
                    |Gabriel|560%\n\
                    |Gabriel|840%\n\
                    |Gabriel|980%\n\
                    |Gabriel|1,120%\n\
                    |Gabriel|1,260%\n\
                    |Gabriel|1,400%\n\
                    |Gabriel|2,800%}}"
                ),
                TemplateParameter::new("boss3", "{{Magnification|Le'boin|14,000%}}"),
            ]
        );
    }

    #[test]
    fn with_separate_mags() {
        let it_25 = Stage::from_id_current(StageID::from_components(T::Tower, 6, 24)).unwrap();
        assert_eq!(
            enemies_list(&it_25, true),
            vec![TemplateParameter::new(
                "enemies",
                "{{Magnification|Pigeon de Sable|300%\n\
                |Elizabeth the LVIth|2,000%\n\
                |Bore Jr.|100%\n\
                |Kory|600%\n\
                |Berserkory|200%\n\
                |Heavy Assault C.A.T.|100/150%\n\
                |Mr. Angel|300%}}"
            )]
        );

        let sacrifice_apprenticeship =
            Stage::from_id_current(StageID::from_components(T::ZL, 3, 3)).unwrap();
        assert_eq!(
            enemies_list(&sacrifice_apprenticeship, true),
            vec![
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|Celeboodle|1,000%\n\
                    |Relic Doge|150%\n\
                    |Sir Rel|150%}}"
                ),
                TemplateParameter::new(
                    "boss",
                    "{{Magnification|Ururun Wolf|300/500%\n\
                    |Mystic Mask Yulala|100%}}"
                )
            ]
        );
    }

    #[test]
    fn simple_4_crown() {
        let sleeping_lion = Stage::from_id_current(StageID::from_components(T::SoL, 0, 7)).unwrap();
        assert_eq!(
            enemies_list(&sleeping_lion, true),
            vec![
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|Doge|400%\n\
                    |Snache|400%\n\
                    |Those Guys|400%\n\
                    |Gory|400%\n\
                    |Hippoe|400%\n\
                    |Doge Dark|100%}}"
                ),
                TemplateParameter::new("boss", "{{Magnification|Squire Rel|100%}}"),
                TemplateParameter::new(
                    "enemies2",
                    "{{Magnification|Doge|600%\n\
                    |Snache|600%\n\
                    |Those Guys|600%\n\
                    |Gory|600%\n\
                    |Hippoe|600%\n\
                    |Doge Dark|150%}}"
                ),
                TemplateParameter::new("boss2", "{{Magnification|Squire Rel|150%}}"),
                TemplateParameter::new(
                    "enemies3",
                    "{{Magnification|Doge|800%\n\
                    |Snache|800%\n\
                    |Those Guys|800%\n\
                    |Gory|800%\n\
                    |Hippoe|800%\n\
                    |Doge Dark|200%}}"
                ),
                TemplateParameter::new("boss3", "{{Magnification|Squire Rel|200%}}"),
                TemplateParameter::new(
                    "enemies4",
                    "{{Magnification|Doge|1,200%\n\
                    |Snache|1,200%\n\
                    |Those Guys|1,200%\n\
                    |Gory|1,200%\n\
                    |Hippoe|1,200%\n\
                    |Doge Dark|300%}}"
                ),
                TemplateParameter::new("boss4", "{{Magnification|Squire Rel|300%}}"),
            ]
        );
    }

    #[test]
    fn with_repeated_enemy() {
        let star_ocean = Stage::from_id_current(StageID::from_components(T::SoL, 15, 7)).unwrap();
        assert_eq!(
            enemies_list(&star_ocean, true),
            [
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|Doge|2,000%\n\
                    |Those Guys|400%\n\
                    |Doge Dark|400%\n\
                    |Doge Dark|500%\n\
                    |Doge Dark|600%\n\
                    |Doge Dark|800%\n\
                    |Doge Dark|1,000%\n\
                    |Doge Dark|1,200%\n\
                    |Doge Dark|2,000%}}"
                ),
                TemplateParameter::new("boss", "{{Magnification|H. Nah|200%}}"),
                TemplateParameter::new(
                    "enemies2",
                    "{{Magnification|Doge|3,000%\n\
                    |Those Guys|600%\n\
                    |Doge Dark|600%\n\
                    |Doge Dark|750%\n\
                    |Doge Dark|900%\n\
                    |Doge Dark|1,200%\n\
                    |Doge Dark|1,500%\n\
                    |Doge Dark|1,800%\n\
                    |Doge Dark|3,000%}}"
                ),
                TemplateParameter::new("boss2", "{{Magnification|H. Nah|300%}}"),
                TemplateParameter::new(
                    "enemies3",
                    "{{Magnification|Doge|4,000%\n\
                    |Those Guys|800%\n\
                    |Doge Dark|800%\n\
                    |Doge Dark|1,000%\n\
                    |Doge Dark|1,200%\n\
                    |Doge Dark|1,600%\n\
                    |Doge Dark|2,000%\n\
                    |Doge Dark|2,400%\n\
                    |Doge Dark|4,000%}}"
                ),
                TemplateParameter::new("boss3", "{{Magnification|H. Nah|400%}}"),
                TemplateParameter::new(
                    "enemies4",
                    "{{Magnification|Doge|4,000%\n\
                    |Those Guys|800%\n\
                    |Doge Dark|800%\n\
                    |Doge Dark|1,000%\n\
                    |Doge Dark|1,200%\n\
                    |Doge Dark|1,600%\n\
                    |Doge Dark|2,000%\n\
                    |Doge Dark|2,400%\n\
                    |Doge Dark|4,000%}}"
                ),
                TemplateParameter::new("boss4", "{{Magnification|H. Nah|400%}}"),
            ]
        );
    }

    #[test]
    fn with_multiple_bosses() {
        let kugel_schreiber =
            Stage::from_id_current(StageID::from_components(T::SoL, 24, 2)).unwrap();
        assert_eq!(
            enemies_list(&kugel_schreiber, true),
            vec![
                TemplateParameter::new("enemies", "{{Magnification|Assassin Bear|200%}}"),
                TemplateParameter::new(
                    "boss",
                    "{{Magnification|Dober P.D|100%\n\
                    |R.Ost|100%\n\
                    |THE SLOTH|200%}}"
                ),
                TemplateParameter::new("enemies2", "{{Magnification|Assassin Bear|240%}}"),
                TemplateParameter::new(
                    "boss2",
                    "{{Magnification|Dober P.D|120%\n\
                    |R.Ost|120%\n\
                    |THE SLOTH|240%}}"
                ),
                TemplateParameter::new("enemies3", "{{Magnification|Assassin Bear|280%}}"),
                TemplateParameter::new(
                    "boss3",
                    "{{Magnification|Dober P.D|140%\n\
                    |R.Ost|140%\n\
                    |THE SLOTH|280%}}"
                ),
                TemplateParameter::new("enemies4", "{{Magnification|Assassin Bear|220%}}"),
                TemplateParameter::new(
                    "boss4",
                    "{{Magnification|Dober P.D|110%\n\
                    |R.Ost|110%\n\
                    |THE SLOTH|220%}}"
                )
            ]
        );
    }

    #[test]
    fn insane_magnifications() {
        let noble_tribe = Stage::from_id_current(StageID::from_components(T::SoL, 43, 2)).unwrap();
        assert_eq!(
            enemies_list(&noble_tribe, true),
            vec![
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|Doge|120,000%\n\
                    |Snache|120,000%\n\
                    |Those Guys|120,000%}}"
                ),
                TemplateParameter::new("boss", "{{Magnification|Hippoe|120,000%}}"),
                TemplateParameter::new(
                    "enemies2",
                    "{{Magnification|Doge|144,000%\n\
                    |Snache|144,000%\n\
                    |Those Guys|144,000%}}"
                ),
                TemplateParameter::new("boss2", "{{Magnification|Hippoe|144,000%}}"),
                TemplateParameter::new(
                    "enemies3",
                    "{{Magnification|Doge|156,000%\n\
                    |Snache|156,000%\n\
                    |Those Guys|156,000%}}"
                ),
                TemplateParameter::new("boss3", "{{Magnification|Hippoe|156,000%}}"),
            ]
        );
    }

    #[test]
    fn floating_point_error_2() {
        // Make sure B.B.Bunny in 3-star doesn't give me 3,919%
        let revenant_road =
            Stage::from_id_current(StageID::from_components(T::SoL, 33, 3)).unwrap();
        assert_eq!(
            enemies_list(&revenant_road, true),
            vec![
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|Zroco|200%\n\
                    |Zir Zeal|200%\n\
                    |Zigge|200%\n\
                    |Zomboe|200%\n\
                    |B.B.Bunny|2,800%}}"
                ),
                TemplateParameter::new("boss", "{{Magnification|Teacher Bun Bun|1,500%}}"),
                TemplateParameter::new(
                    "enemies2",
                    "{{Magnification|Zroco|240%\n\
                    |Zir Zeal|240%\n\
                    |Zigge|240%\n\
                    |Zomboe|240%\n\
                    |B.B.Bunny|3,360%}}"
                ),
                TemplateParameter::new("boss2", "{{Magnification|Teacher Bun Bun|1,800%}}"),
                TemplateParameter::new(
                    "enemies3",
                    "{{Magnification|Zroco|280%\n\
                    |Zir Zeal|280%\n\
                    |Zigge|280%\n\
                    |Zomboe|280%\n\
                    |B.B.Bunny|3,920%}}"
                ),
                TemplateParameter::new("boss3", "{{Magnification|Teacher Bun Bun|2,100%}}"),
            ]
        );
    }

    #[test]
    fn with_base() {
        let finale = Stage::from_id_current(StageID::from_components(T::Collab, 209, 0)).unwrap();
        assert_eq!(
            enemies_list(&finale, true),
            vec![TemplateParameter::new(
                "base",
                "{{Magnification|Finale Base|100%}}"
            ),]
        );
    }

    #[test]
    fn with_insane_base() {
        let relay_1600m =
            Stage::from_id_current(StageID::from_components(T::Extra, 61, 2)).unwrap();
        assert_eq!(
            enemies_list(&relay_1600m, true),
            vec![
                TemplateParameter::new("base", "{{Magnification|Relay Base|7,500,000%}}"),
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|White Wind|700%\n\
                    |Duche|300%\n\
                    |Red Wind|700%\n\
                    |Gory Black|200%\n\
                    |Black Wind|700%\n\
                    |R.Ost|100%\n\
                    |Bore|200%}}"
                ),
                TemplateParameter::new("boss", "{{Magnification|Le'noir|150%}}"),
            ]
        );
    }

    #[test]
    fn with_weird_base() {
        // basically just here for same reasons it was in information's tests
        let pile_of_guts = Stage::from_id_current(StageID::from_components(T::UL, 31, 5)).unwrap();
        assert_eq!(
            enemies_list(&pile_of_guts, true),
            vec![
                TemplateParameter::new("base", "{{Magnification|Relic Doge Base|40,000%}}"),
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|Bore Jr.|100%\n\
                    |Celeboodle|1,000%\n\
                    |R.Ost|300%}}"
                ),
                TemplateParameter::new("boss", "{{Magnification|THE SLOTH|400%}}"),
                TemplateParameter::new("base2", "{{Magnification|Relic Doge Base|52,000%}}"),
                TemplateParameter::new(
                    "enemies2",
                    "{{Magnification|Bore Jr.|130%\n\
                    |Celeboodle|1,300%\n\
                    |R.Ost|390%}}"
                ),
                TemplateParameter::new("boss2", "{{Magnification|THE SLOTH|520%}}"),
                TemplateParameter::new("base3", "{{Magnification|Relic Doge Base|68,000%}}"),
                TemplateParameter::new(
                    "enemies3",
                    "{{Magnification|Bore Jr.|170%\n\
                    |Celeboodle|1,700%\n\
                    |R.Ost|510%}}"
                ),
                TemplateParameter::new("boss3", "{{Magnification|THE SLOTH|680%}}"),
            ]
        );
    }

    #[test]
    fn gauntlet_suppress() {
        let baron_seal =
            Stage::from_id_current(StageID::from_components(T::Gauntlet, 20, 0)).unwrap();
        assert_eq!(
            enemies_list(&baron_seal, true),
            vec![
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|Doge Dark|0\n\
                    |Zoge|0\n\
                    |Gory Black|0\n\
                    |Zory|0\n\
                    |Shadow Boxer K|0\n\
                    |Zang Roo|0}}"
                ),
                TemplateParameter::new("boss", "{{Magnification|Baron Seal|0}}"),
            ]
        );
    }

    #[test]
    fn gauntlet_no_suppress() {
        let baron_seal =
            Stage::from_id_current(StageID::from_components(T::Gauntlet, 20, 0)).unwrap();
        assert_eq!(
            enemies_list(&baron_seal, false),
            vec![
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|Doge Dark|500%\n\
                    |Zoge|150%\n\
                    |Gory Black|150%\n\
                    |Zory|100%\n\
                    |Shadow Boxer K|150%\n\
                    |Zang Roo|100%}}"
                ),
                TemplateParameter::new("boss", "{{Magnification|Baron Seal|6,000%}}"),
            ]
        );
    }

    #[test]
    fn dojo() {
        let wanderer_trial =
            Stage::from_id_current(StageID::from_components(T::Dojo, 0, 0)).unwrap();
        assert_eq!(
            enemies_list(&wanderer_trial, true),
            vec![
                TemplateParameter::new("base", "{{Magnification|Scarecrow|0}}"),
                TemplateParameter::new(
                    "enemies",
                    "{{Magnification|One Horn|0\n\
                    |Doge Dark|0\n\
                    |St. Pigge the 2nd|0\n\
                    |Squire Rel|0\n\
                    |R.Ost|0\n\
                    |Shadow Boxer K|0\n\
                    |Dagshund|0\n\
                    |Le'boin|0}}"
                ),
                TemplateParameter::new(
                    "boss",
                    "{{Magnification|The Face|0\n\
                    |Squire Rel|0\n\
                    |R.Ost|0\n\
                    |St. Pigge the 2nd|0\n\
                    |Le'boin|0}}"
                ),
            ]
        );
        assert_eq!(
            enemies_list(&wanderer_trial, true),
            enemies_list(&wanderer_trial, false),
        );
    }
}
