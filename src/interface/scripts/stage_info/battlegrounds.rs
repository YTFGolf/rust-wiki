//! Get the battlegrounds section of stage.

use crate::{
    game_data::{
        meta::stage::variant::StageVariantID as T,
        stage::parsed::{
            stage::Stage,
            stage_enemy::{BossType, EnemyAmount, MS_SIGN, StageEnemy},
        },
    },
    interface::error_handler::InfallibleWrite,
    wiki_data::enemy_data::ENEMY_DATA,
    wikitext::wiki_utils::{extract_name, get_precision_f},
};
use either::Either::{Left, Right};
use num_format::{Locale, WriteFormatted};
use regex::Regex;
use std::fmt::Write;

/// Write the given spawn time in seconds;
fn write_single_spawn_s(buf: &mut String, time_f: u32) {
    let respawn_s = f64::from(time_f) / 30.0;
    assert!(respawn_s < 1_000.0, "Spawn time is above 1,000 seconds!");
    let precision = get_precision_f(time_f);
    write!(buf, "{respawn_s:.precision$}").unwrap();
}

/// Write the enemy delay part of the battlegrounds lines.
fn write_enemy_delay(buf: &mut String, enemy: &StageEnemy) {
    *buf += ", delay ";

    write_single_spawn_s(buf, enemy.respawn_time.0);
    if enemy.respawn_time.1 > enemy.respawn_time.0 {
        *buf += "~";
        write_single_spawn_s(buf, enemy.respawn_time.1);
    }

    if enemy.respawn_time == (30, 30) {
        *buf += " second";
    } else {
        *buf += " seconds";
    }

    *buf += "<sup>";
    buf.write_formatted(&enemy.respawn_time.0, &Locale::en)
        .unwrap();
    *buf += "f";
    if enemy.respawn_time.1 > enemy.respawn_time.0 {
        *buf += "~";
        buf.write_formatted(&enemy.respawn_time.1, &Locale::en)
            .unwrap();
        *buf += "f";
    }
    *buf += "</sup>";
}

/// Matcher string for all enemy bases that should begin with "is a*n*" instead
/// of "is a".
const AN_ENEMY_MATCHER: &str = r"^([AEIOU]|11|18|8)";

/// Get the battlegrounds line for a single enemy.
fn get_single_enemy_line(
    enemy: &StageEnemy,
    is_base_hit: bool,
    show_magnification: bool,
) -> String {
    let mut buf = String::new();

    if is_base_hit {
        buf += "**";
    } else {
        buf += "*";
    }

    if enemy.is_base {
        let name = &ENEMY_DATA.get_names(enemy.id).name;
        let an = match Regex::new(AN_ENEMY_MATCHER)
            .unwrap()
            .is_match(extract_name(name))
        {
            true => "an",
            false => "a",
        };

        write!(buf, "The enemy base here is {an} {name}.").unwrap();
        return buf;
    }

    match enemy.amount {
        EnemyAmount::Infinite => buf += "Infinite",
        EnemyAmount::Limit(n) => {
            buf.write_formatted(&n, &Locale::en).infallible_write();
        }
    }

    let is_single_enemy: bool = enemy.amount.is_singular();
    if is_single_enemy {
        write!(buf, " {}", ENEMY_DATA.get_names(enemy.id).name).unwrap();
    } else {
        write!(buf, " {}", ENEMY_DATA.get_names(enemy.id).plural).unwrap();
    }
    if show_magnification {
        match enemy.magnification {
            Left(mag) => {
                buf += " (";
                buf.write_formatted(&mag, &Locale::en).infallible_write();
                buf += "%)";
            }
            Right((hp, ap)) => {
                buf += " (";
                buf.write_formatted(&hp, &Locale::en).infallible_write();
                buf += "% HP, ";
                buf.write_formatted(&ap, &Locale::en).infallible_write();
                buf += "% AP)";
            }
        }
    }

    if is_single_enemy {
        buf += " spawns";
    } else {
        buf += " spawn";
    }

    if enemy.boss_type != BossType::None {
        match is_single_enemy {
            true => buf += " as the boss",
            false => buf += " as bosses",
        }
    }

    let is_instant_spawn = enemy.start_frame == 0 || (is_base_hit && !enemy.enforce_start_frame);
    if !is_instant_spawn {
        buf += " after ";
        write_single_spawn_s(&mut buf, enemy.start_frame);
        if enemy.start_frame == 30 {
            buf += " second<sup>";
        } else {
            buf += " seconds<sup>";
        }
        buf.write_formatted(&enemy.start_frame, &Locale::en)
            .unwrap();
        buf += "f</sup>";
    }

    if let Some(kills) = enemy.kill_count {
        assert!(
            u32::from(kills) < 1_000,
            "Cat kill count is more than 1,000!"
        );
        write!(buf, " once {kills} Cat Units have been defeated").unwrap();
    }

    if !is_single_enemy {
        write_enemy_delay(&mut buf, enemy);
    }

    buf.write_char('.').infallible_write();

    buf
}

/// (base hit value, enemies that spawn at that base hit)
type OtherSpawnItem<'a> = (u32, Vec<&'a StageEnemy>);
/// Get enemies that spawn immediately, enemies that spawn after a certain
/// amount of base hit, and enemies that appear multiple times at different
/// magnifications.
fn get_enemy_spawns(
    stage: &Stage,
    is_default_spawn: fn(&StageEnemy) -> bool,
    is_dojo: bool,
) -> (Vec<&StageEnemy>, Vec<OtherSpawnItem>, Vec<u32>) {
    let mut default_spawn: Vec<&StageEnemy> = vec![];
    let mut other_spawn: Vec<OtherSpawnItem> = vec![];
    let mut enemies_mags = vec![];
    let mut enemies_dupe = vec![];
    for enemy in &stage.enemies {
        if let Some((_id, mag)) = enemies_mags.iter().find(|(id, _mag)| *id == enemy.id) {
            if *mag != enemy.magnification {
                enemies_dupe.push(enemy.id);
            }
        } else {
            enemies_mags.push((enemy.id, enemy.magnification));
        }

        if is_default_spawn(enemy) {
            default_spawn.push(enemy);
            continue;
        }

        if let Some((_, percentage_vec)) = other_spawn
            .iter_mut()
            .find(|(percent, _)| *percent == enemy.base_hp)
        {
            percentage_vec.push(enemy);
        } else {
            other_spawn.push((enemy.base_hp, vec![enemy]));
        }
    }
    let order_function: fn(&OtherSpawnItem, &OtherSpawnItem) -> std::cmp::Ordering = match is_dojo {
        false => |a, b| b.0.cmp(&a.0),
        true => |a, b| a.0.cmp(&b.0),
    };
    other_spawn.sort_by(order_function);

    default_spawn.sort_by_key(|e| !e.is_base);
    other_spawn
        .iter_mut()
        .for_each(|l| l.1.sort_by_key(|e| e.boss_type == BossType::None));
    (default_spawn, other_spawn, enemies_dupe)
}

/// Get the battlegrounds section of the stage.
pub fn battlegrounds(stage: &Stage) -> String {
    let is_dojo = matches!(stage.id.variant(), T::Dojo | T::RankingDojo);

    let is_default_spawn: fn(&StageEnemy) -> bool = match is_dojo {
        false => |enemy: &StageEnemy| enemy.base_hp >= 100,
        true => |enemy: &StageEnemy| enemy.base_hp == 0,
    };

    let (default_spawn, other_spawn, enemies_dupe) =
        get_enemy_spawns(stage, is_default_spawn, is_dojo);

    // this is not an abstraction, this is a convenience. having a bool here
    // only works because I always know it's a bool
    fn stringify_enemy_list(
        enemies: &[&StageEnemy],
        is_base_hit: bool,
        enemies_dupe: &[u32],
    ) -> String {
        enemies
            .iter()
            .filter_map(|e| {
                if e.id == MS_SIGN && e.start_frame == 27_000 && e.boss_type == BossType::None {
                    None
                } else {
                    Some(get_single_enemy_line(
                        e,
                        is_base_hit,
                        enemies_dupe.contains(&e.id),
                    ))
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    let mut buf = String::new();
    if stage.is_base_indestructible {
        buf += "*The enemy base is indestructible until the boss is defeated.\n";
    }
    buf += &stringify_enemy_list(&default_spawn, false, &enemies_dupe);

    for other in other_spawn {
        if !is_dojo {
            if other.0 == 0 {
                continue;
            }
            if !buf.is_empty() {
                buf += "\n";
            }
            writeln!(buf, "*When the base reaches {hp}% HP:", hp = other.0).unwrap();
            buf += &stringify_enemy_list(&other.1, true, &enemies_dupe);
            continue;
        }

        buf += "\n*When the base takes ";
        buf.write_formatted(&other.0, &Locale::en)
            .infallible_write();
        buf += " damage:\n";
        buf += &stringify_enemy_list(&other.1, true, &enemies_dupe);
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_data::meta::stage::stage_id::StageID;

    #[test]
    fn test_basic_battleground() {
        let earthshaker = Stage::from_id_current(StageID::from_components(T::SoL, 0, 0)).unwrap();
        assert_eq!(earthshaker.enemies.len(), 4);
        assert_eq!(earthshaker.enemies[3].start_frame, 27_000);
        assert_eq!(
            battlegrounds(&earthshaker),
            "*50 [[Doge]]s spawn, delay 2~6 seconds<sup>60f~180f</sup>.\n\
            *50 [[Snache]]s spawn, delay 2~6 seconds<sup>60f~180f</sup>.\n\
            *50 [[Those Guys]] spawn, delay 2~6 seconds<sup>60f~180f</sup>."
        );
    }

    #[test]
    fn test_show_mag_and_ms_sign() {
        let star_ocean = Stage::from_id_current(StageID::from_components(T::SoL, 15, 7)).unwrap();
        assert_eq!(
            battlegrounds(&star_ocean),
            "*Infinite [[Doge]]s spawn, delay 4~40 seconds<sup>120f~1,200f</sup>.\n\
            *Infinite [[Those Guys]] spawn, delay 0.07~2 seconds<sup>2f~60f</sup>.\n\
            *Infinite [[Doge Dark]]s (400%) spawn after 66.67 seconds<sup>2,000f</sup>, delay 0.07~10 seconds<sup>2f~300f</sup>.\n\
            *Infinite [[Doge Dark]]s (500%) spawn after 100 seconds<sup>3,000f</sup>, delay 0.07~8 seconds<sup>2f~240f</sup>.\n\
            *Infinite [[Doge Dark]]s (600%) spawn after 133.33 seconds<sup>4,000f</sup>, delay 0.07~5.33 seconds<sup>2f~160f</sup>.\n\
            *Infinite [[Doge Dark]]s (800%) spawn after 166.67 seconds<sup>5,000f</sup>, delay 0.07~2.67 seconds<sup>2f~80f</sup>.\n\
            *Infinite [[Doge Dark]]s (1,000%) spawn after 200 seconds<sup>6,000f</sup>, delay 0.07~1.33 seconds<sup>2f~40f</sup>.\n\
            *Infinite [[Doge Dark]]s (1,200%) spawn after 233.33 seconds<sup>7,000f</sup>, delay 0.07~0.67 seconds<sup>2f~20f</sup>.\n\
            *Infinite [[Doge Dark]]s (2,000%) spawn after 266.67 seconds<sup>8,000f</sup>, delay 0.07 seconds<sup>2f</sup>.\n\
            *Infinite [[Ms. Sign]]s spawn after 200 seconds<sup>6,000f</sup>, delay 66.67 seconds<sup>2,000f</sup>.\n\
            *When the base reaches 90% HP:\n\
            **1 [[H. Nah]] spawns as the boss.\n\
            **Infinite [[Doge Dark]]s (400%) spawn, delay 2~6.6 seconds<sup>60f~198f</sup>."
        );
    }

    #[test]
    fn test_base_hit() {
        let lovely_minerals =
            Stage::from_id_current(StageID::from_components(T::SoL, 4, 0)).unwrap();
        let master_a = &lovely_minerals.enemies[4];
        assert_eq!(master_a.start_frame, 1_200);
        assert_eq!(master_a.base_hp, 90);
        assert!(!master_a.enforce_start_frame);
        assert_eq!(
            battlegrounds(&lovely_minerals),
            "*Infinite [[Croco]]s spawn, delay 3~20 seconds<sup>90f~600f</sup>.\n\
            *Infinite [[Snache]]s spawn, delay 3~20 seconds<sup>90f~600f</sup>.\n\
            *Infinite [[Those Guys]] spawn after 40 seconds<sup>1,200f</sup>, delay 3~20 seconds<sup>90f~600f</sup>.\n\
            *Infinite [[Gory|Gories]] spawn after 40 seconds<sup>1,200f</sup>, delay 3~20 seconds<sup>90f~600f</sup>.\n\
            *When the base reaches 90% HP:\n\
            **1 [[Master A.]] spawns.\n\
            **1 [[Dagshund]] spawns.\n\
            **1 [[Dagshund]] spawns."
        );
    }

    #[test]
    fn test_zero_percent() {
        let way_of_sleeping_punt =
            Stage::from_id_current(StageID::from_components(T::Collab, 44, 0)).unwrap();
        assert_eq!(way_of_sleeping_punt.enemies[5].base_hp, 0);
        assert_eq!(
            battlegrounds(&way_of_sleeping_punt),
            "*5 [[Hippoe]]s spawn, delay 33.33~66.67 seconds<sup>1,000f~2,000f</sup>.\n\
            *Infinite [[Gory|Gories]] spawn after 16.67 seconds<sup>500f</sup>, delay 33.33~66.67 seconds<sup>1,000f~2,000f</sup>.\n\
            *When the base reaches 99% HP:\n\
            **1 [[Oversleeping Beauty Punt (Enemy)|Oversleeping Beauty Punt]] spawns as the boss.\n\
            **Infinite [[Pigge]]s spawn, delay 40~80 seconds<sup>1,200f~2,400f</sup>.\n\
            **10 [[Sir Seal]]s spawn, delay 40~80 seconds<sup>1,200f~2,400f</sup>."
        );
    }

    #[test]
    fn test_multibase_with_1_second() {
        let mistakes_dont_matter =
            Stage::from_id_current(StageID::from_components(T::Event, 261, 3)).unwrap();
        let base = &mistakes_dont_matter.enemies[0];
        assert_eq!(base.amount, EnemyAmount::from(10));
        assert!(base.is_base);
        assert_eq!(
            battlegrounds(&mistakes_dont_matter),
            "*The enemy base here is a [[Doge Base (Enemy Base)|Doge Base]].\n\
            *5 [[Pigeon de Sable]]s spawn after 10 seconds<sup>300f</sup>, delay 0.53~1.13 seconds<sup>16f~34f</sup>.\n\
            *When the base reaches 95% HP:\n\
            **1 [[Snowball]] spawns as the boss.\n\
            **7 [[Wafer Cat XL (Enemy)|Wafer Cat XL]]s spawn, delay 1 second<sup>30f</sup>.\n\
            *When the base reaches 90% HP:\n\
            **1 [[Snowball]] spawns as the boss.\n\
            **3 [[Ice Doge]]s spawn, delay 3.33 seconds<sup>100f</sup>.\n\
            **1 [[Chocolate Doge]] spawns.\n\
            *When the base reaches 85% HP:\n\
            **1 [[Snowball]] spawns as the boss.\n\
            **1 [[Chocolate Doge]] spawns.\n\
            **3 [[Ice Doge]]s spawn, delay 3.33 seconds<sup>100f</sup>.\n\
            **5 [[Wall Doge]]s spawn, delay 3.33 seconds<sup>100f</sup>."
        );
        // good candidate for ordering tests
    }

    #[test]
    fn test_1_second_spawn() {
        let cat_catharsis =
            Stage::from_id_current(StageID::from_components(T::SoL, 27, 2)).unwrap();
        assert_eq!(
            battlegrounds(&cat_catharsis),
            "*1 [[Dark Emperor Nyandam]] spawns after 1 second<sup>30f</sup>.\n\
            *1 [[Director Kurosawah]] spawns after 33.33 seconds<sup>1,000f</sup>.\n\
            *1 [[Galactic Overseer Nyandam]] spawns as the boss after 66.67 seconds<sup>2,000f</sup>.\n\
            *1 [[Kory]] spawns after 4 seconds<sup>120f</sup>.\n\
            *1 [[Berserkory]] spawns after 80 seconds<sup>2,400f</sup>."
        );
    }

    #[test]
    fn test_multiboss() {
        let ultra_stress =
            Stage::from_id_current(StageID::from_components(T::Event, 34, 0)).unwrap();
        let squirrels = &ultra_stress.enemies[5];
        assert_eq!(squirrels.amount, EnemyAmount::from(5));
        assert_eq!(squirrels.boss_type, BossType::Boss);
        assert_eq!(
            battlegrounds(&ultra_stress),
            "*Infinite [[Fireworks Guys (High-Yield)]] spawn, delay 10~16.67 seconds<sup>300f~500f</sup>.\n\
            *Infinite [[Fireworks Guys (Low-Yield)]] spawn after 3.33 seconds<sup>100f</sup>, delay 10~16.67 seconds<sup>300f~500f</sup>.\n\
            *Infinite [[Chief Peng]]s spawn after 80 seconds<sup>2,400f</sup>, delay 10~16.67 seconds<sup>300f~500f</sup>.\n\
            *4 [[Chief Peng]]s spawn after 26.67 seconds<sup>800f</sup>, delay 0.67~2 seconds<sup>20f~60f</sup>.\n\
            *1 [[Chief Peng]] spawns as the boss after 6.67 seconds<sup>200f</sup>.\n\
            *5 [[Squire Rel]]s spawn as bosses after 53.33 seconds<sup>1,600f</sup>, delay 6.67 seconds<sup>200f</sup>.\n\
            *1 [[Dark Emperor Santa]] spawns as the boss after 40 seconds<sup>1,200f</sup>.\n\
            *1 [[Lucky Sloth]] spawns as the boss after 46.67 seconds<sup>1,400f</sup>.\n\
            *3 [[Shy Boy]]s spawn after 133.33 seconds<sup>4,000f</sup>, delay 6.67 seconds<sup>200f</sup>.\n\
            *1 [[Kory]] spawns as the boss after 213.33 seconds<sup>6,400f</sup>."
        );
    }

    #[test]
    fn test_base_out_of_order() {
        let bouquet_toss =
            Stage::from_id_current(StageID::from_components(T::Event, 343, 4)).unwrap();
        let base = &bouquet_toss.enemies[5];
        assert!(base.is_base);
        assert_eq!(
            battlegrounds(&bouquet_toss),
            "*The enemy base here is a [[Clown Base (Enemy Base)|Clown Base]].\n\
            *3 [[Gory Black]]s spawn after 13.33 seconds<sup>400f</sup>, delay 13.33~20 seconds<sup>400f~600f</sup>.\n\
            *When the base reaches 99% HP:\n\
            **1 [[Le'noir]] spawns as the boss.\n\
            **5 [[Doge Dark]]s spawn, delay 6.67~10 seconds<sup>200f~300f</sup>.\n\
            **3 [[Shadow Boxer K]]s spawn, delay 20~23.33 seconds<sup>600f~700f</sup>.\n\
            **5 [[Doge Dark]]s spawn after 13.33 seconds<sup>400f</sup>, delay 13.33~16.67 seconds<sup>400f~500f</sup>."
        );
        // use this one as basis for ordering tests
    }

    #[test]
    fn test_indestructible_base() {
        let disaster_strikes =
            Stage::from_id_current(StageID::from_components(T::Event, 369, 0)).unwrap();
        assert!(disaster_strikes.is_base_indestructible);
        assert_eq!(
            battlegrounds(&disaster_strikes),
            "*The enemy base is indestructible until the boss is defeated.\n\
            *1 [[Baa Baa]] spawns after 0.67 seconds<sup>20f</sup>.\n\
            *2 [[Jackie Peng]]s spawn after 3.33 seconds<sup>100f</sup>, delay 30 seconds<sup>900f</sup>.\n\
            *1 [[Mysterious Calamity]] spawns as the boss after 20 seconds<sup>600f</sup>.\n\
            *Infinite [[Baa Baa]]s spawn after 40 seconds<sup>1,200f</sup>, delay 46.67~53.33 seconds<sup>1,400f~1,600f</sup>.\n\
            *2 [[Kang Roo]]s spawn after 53.33 seconds<sup>1,600f</sup>, delay 60 seconds<sup>1,800f</sup>.\n\
            *Infinite [[Jackie Peng]]s spawn after 86.67 seconds<sup>2,600f</sup>, delay 36.67~40 seconds<sup>1,100f~1,200f</sup>."
        );
    }

    #[test]
    fn test_enforce_start_frame_1() {
        let retreat_of_living_dead =
            Stage::from_id_current(StageID::from_components(T::UL, 45, 3)).unwrap();
        assert_eq!(
            battlegrounds(&retreat_of_living_dead),
            "*The enemy base here is a [[Surge Base (Enemy Base)|Surge Base]].\n\
            *4 [[Relic Doge]]s spawn after 10 seconds<sup>300f</sup>, delay 20~23.33 seconds<sup>600f~700f</sup>.\n\
            *1 [[Zang Roo]] spawns after 36.67 seconds<sup>1,100f</sup>.\n\
            *When the base reaches 99% HP:\n\
            **1 [[Golem Sunfish]] spawns as the boss.\n\
            **Infinite [[Relic Doge]]s spawn, delay 33.33~40 seconds<sup>1,000f~1,200f</sup>.\n\
            **8 [[Relic Doge]]s spawn, delay 10 seconds<sup>300f</sup>.\n\
            **2 [[Zang Roo]]s spawn after 13.33 seconds<sup>400f</sup>, delay 53.33~56.67 seconds<sup>1,600f~1,700f</sup>.\n\
            **3 [[Zapy|Zapies]] spawn, delay 33.33~36.67 seconds<sup>1,000f~1,100f</sup>."
        );
    }

    #[test]
    fn test_enforce_start_frame_with_1_second() {
        let titanic_steakhouse =
            Stage::from_id_current(StageID::from_components(T::UL, 45, 4)).unwrap();
        assert_eq!(
            battlegrounds(&titanic_steakhouse),
            "*10 [[Cerberus Kids]] spawn after 6.67 seconds<sup>200f</sup>, delay 2~4 seconds<sup>60f~120f</sup>.\n\
            *20 [[Cerberus Kids]] spawn after 13.33 seconds<sup>400f</sup>, delay 1 second<sup>30f</sup>.\n\
            *1 [[Relic Doge]] spawns after 20 seconds<sup>600f</sup>.\n\
            *5 [[Aku Doge]]s spawn after 23.33 seconds<sup>700f</sup>, delay 33.33~36.67 seconds<sup>1,000f~1,100f</sup>.\n\
            *8 [[Cerberus Kids]] spawn after 80 seconds<sup>2,400f</sup>, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            *2 [[Relic Doge]]s spawn after 100 seconds<sup>3,000f</sup>, delay 23.33~26.67 seconds<sup>700f~800f</sup>.\n\
            *Infinite [[Cerberus Kids]] spawn after 120 seconds<sup>3,600f</sup>, delay 13.33~20 seconds<sup>400f~600f</sup>.\n\
            *3 [[Relic Doge]]s spawn after 133.33 seconds<sup>4,000f</sup>, delay 23.33~26.67 seconds<sup>700f~800f</sup>.\n\
            *8 [[Cerberus Kids]] spawn after 80 seconds<sup>2,400f</sup>, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            *Infinite [[Relic Doge]]s spawn after 233.33 seconds<sup>7,000f</sup>, delay 56.67~60 seconds<sup>1,700f~1,800f</sup>.\n\
            *When the base reaches 99% HP:\n\
            **1 [[Othom]] spawns as the boss.\n\
            **1 [[Deathkory]] spawns.\n\
            **1 [[Ackey]] spawns after 36.67 seconds<sup>1,100f</sup>."
        );
    }

    #[test]
    fn test_an() {
        let mexico = Stage::from_id_current(StageID::from_components(T::AkuRealms, 0, 42)).unwrap();
        let base_name = ENEMY_DATA.get_common_name(u32::from(mexico.anim_base_id.unwrap()) - 2);
        assert_eq!(base_name.chars().next().unwrap(), 'A');
        assert_eq!(
            battlegrounds(&mexico),
            "*The enemy base here is an [[Aku Altar (Enemy Base)|Aku Altar]].\n\
            *1 [[Gabriel]] spawns after 10 seconds<sup>300f</sup>.\n\
            *1 [[Gabriel]] spawns after 20 seconds<sup>600f</sup>.\n\
            *1 [[Gabriel]] spawns after 20.67 seconds<sup>620f</sup>.\n\
            *1 [[Gabriel]] spawns after 21.33 seconds<sup>640f</sup>.\n\
            *1 [[Gabriel]] spawns after 40 seconds<sup>1,200f</sup>.\n\
            *1 [[Gabriel]] spawns after 40.67 seconds<sup>1,220f</sup>.\n\
            *1 [[Gabriel]] spawns after 41.67 seconds<sup>1,250f</sup>.\n\
            *1 [[Gabriel]] spawns after 43 seconds<sup>1,290f</sup>.\n\
            *1 [[Gabriel]] spawns after 44.67 seconds<sup>1,340f</sup>.\n\
            *When the base reaches 99% HP:\n\
            **1 [[Sunfish Jones]] spawns as the boss.\n\
            **Infinite [[Gabriel]]s spawn, delay 10~20 seconds<sup>300f~600f</sup>.\n\
            **15 [[Gabriel]]s spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **5 [[Fallen Bear]]s spawn, delay 10 seconds<sup>300f</sup>."
        );
    }

    #[test]
    fn test_infinite_base() {
        let stand_by_motel =
            Stage::from_id_current(StageID::from_components(T::UL, 45, 1)).unwrap();
        let base = &stand_by_motel.enemies[0];
        assert_eq!(base.amount, EnemyAmount::Infinite);
        assert!(base.is_base);
        assert_eq!(
            battlegrounds(&stand_by_motel),
            "*The enemy base here is a [[Doge Base (Enemy Base)|Doge Base]].\n\
            *Infinite [[Aku Doge]]s spawn after 10 seconds<sup>300f</sup>, delay 23.33~33.33 seconds<sup>700f~1,000f</sup>.\n\
            *1 [[Aku Doge]] spawns after 30 seconds<sup>900f</sup>.\n\
            *3 [[Aku Doge]]s spawn after 60 seconds<sup>1,800f</sup>, delay 1~2 seconds<sup>30f~60f</sup>.\n\
            *When the base reaches 99% HP:\n\
            **1 [[Aku Doge]] spawns as the boss.\n\
            **15 [[Aku Doge]]s spawn, delay 6.67~10 seconds<sup>200f~300f</sup>.\n\
            **5 [[Aku Gory|Aku Gories]] spawn, delay 20~23.33 seconds<sup>600f~700f</sup>.\n\
            **2 [[Oldhorn]]s spawn, delay 20 seconds<sup>600f</sup>.\n\
            **1 [[Midnite D.]] spawns."
        );
    }

    #[test]
    fn test_ms_sign_boss() {
        let hall_of_four_kings =
            Stage::from_id_current(StageID::from_components(T::UL, 36, 5)).unwrap();
        let ms_sign = &hall_of_four_kings.enemies[1];
        assert_eq!(ms_sign.id, MS_SIGN);
        assert_eq!(ms_sign.boss_type, BossType::Boss);
        assert_eq!(
            battlegrounds(&hall_of_four_kings),
            "*1 [[Assassin Bear]] spawns after 233.33 seconds<sup>7,000f</sup>.\n\
            *1 [[Ms. Sign]] spawns as the boss after 233.33 seconds<sup>7,000f</sup>.\n\
            *1 [[St. Dober]] spawns as the boss after 33.33 seconds<sup>1,000f</sup>.\n\
            *1 [[M. Ost]] spawns as the boss after 33.33 seconds<sup>1,000f</sup>.\n\
            *1 [[Elder Sloth]] spawns as the boss after 3.33 seconds<sup>100f</sup>."
        );
    }

    #[test]
    fn test_killcount() {
        let deep_jungle_10 =
            Stage::from_id_current(StageID::from_components(T::Behemoth, 0, 9)).unwrap();
        assert_eq!(
            battlegrounds(&deep_jungle_10),
            "*Infinite [[Doge Dark]]s spawn after 6.67 seconds<sup>200f</sup>, delay 13.33~16.67 seconds<sup>400f~500f</sup>.\n\
            *1 [[THE FOLIVOREAN]] spawns as the boss after 20 seconds<sup>600f</sup>.\n\
            *5 [[Wild Doge]]s spawn after 26.67 seconds<sup>800f</sup>, delay 26.67~30 seconds<sup>800f~900f</sup>.\n\
            *2 [[Ragin' Gory|Ragin' Gories]] spawn after 46.67 seconds<sup>1,400f</sup>, delay 30~33.33 seconds<sup>900f~1,000f</sup>.\n\
            *Infinite [[Wild Doge]]s spawn after 56.67 seconds<sup>1,700f</sup>, delay 33.33~40 seconds<sup>1,000f~1,200f</sup>.\n\
            *2 [[Ragin' Gory|Ragin' Gories]] spawn once 60 Cat Units have been defeated, delay 20~23.33 seconds<sup>600f~700f</sup>.\n\
            *2 [[Ragin' Gory|Ragin' Gories]] spawn once 120 Cat Units have been defeated, delay 20~23.33 seconds<sup>600f~700f</sup>."
        );
    }

    #[test]
    fn test_all_enemies_base_hit() {
        let great_burglar_battle =
            Stage::from_id_current(StageID::from_components(T::Collab, 132, 0)).unwrap();
        assert_eq!(
            battlegrounds(&great_burglar_battle),
            "*When the base reaches 99% HP:\n\
            **1 [[Rat Doge]] (1%) spawns as the boss.\n\
            **5 [[Rat Doge]]s (50%) spawn, delay 8~12 seconds<sup>240f~360f</sup>.\n\
            **10 [[Rat Doge]]s (50%) spawn, delay 4~6 seconds<sup>120f~180f</sup>.\n\
            **15 [[Rat Doge]]s (50%) spawn, delay 2~3 seconds<sup>60f~90f</sup>."
        );
    }

    #[test]
    fn test_dojo() {
        let anniv_11 =
            Stage::from_id_current(StageID::from_components(T::RankingDojo, 25, 0)).unwrap();
        // 11th anniversary is plural + general dojo layout
        // nearly 200 lines lmao
        assert_eq!(
            battlegrounds(&anniv_11),
            "*The enemy base here is an [[11th Anniversary Scarecrow (Enemy Base)|11th Anniversary Scarecrow]].\n\
            *Infinite [[Fireworks Guys (High-Yield)]] spawn after 3.33 seconds<sup>100f</sup>, delay 8~10 seconds<sup>240f~300f</sup>.\n\
            *When the base takes 1,000 damage:\n\
            **1 [[Le'boin]] (100%) spawns as the boss.\n\
            **3 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            *When the base takes 3,000 damage:\n\
            **1 [[One Horn]] (100%) spawns.\n\
            *When the base takes 7,500 damage:\n\
            **1 [[Le'boin]] (100%) spawns.\n\
            *When the base takes 15,000 damage:\n\
            **1 [[The Face]] (100%) spawns as the boss.\n\
            **5 [[Doge Dark]]s (100%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            **1 [[One Horn]] (100%) spawns.\n\
            **3 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            *When the base takes 30,000 damage:\n\
            **1 [[Le'boin]] (200%) spawns.\n\
            **1 [[Fireworks Guys (High-Yield)]] spawns.\n\
            *When the base takes 45,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (200%) spawns as the boss.\n\
            **5 [[Doge Dark]]s (200%) spawn, delay 3.33~5 seconds<sup>100f~150f</sup>.\n\
            **3 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            *When the base takes 60,000 damage:\n\
            **1 [[Fireworks Guys (High-Yield)]] spawns.\n\
            **1 [[One Horn]] (400%) spawns.\n\
            *When the base takes 90,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (400%) spawns as the boss.\n\
            **5 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **1 [[Dagshund]] (200%) spawns.\n\
            **1 [[Dagshund]] (200%) spawns.\n\
            **1 [[Dagshund]] (200%) spawns.\n\
            *When the base takes 120,000 damage:\n\
            **1 [[Shadow Boxer K]] (100%) spawns.\n\
            *When the base takes 150,000 damage:\n\
            **1 [[Shadow Boxer K]] (100%) spawns.\n\
            *When the base takes 180,000 damage:\n\
            **1 [[The Face]] (400%) spawns as the boss.\n\
            **5 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **5 [[Doge Dark]]s (400%) spawn, delay 3.33~5 seconds<sup>100f~150f</sup>.\n\
            **5 [[Doge Dark]]s (400%) spawn, delay 6.67~10 seconds<sup>200f~300f</sup>.\n\
            *When the base takes 210,000 damage:\n\
            **1 [[Le'boin]] (600%) spawns.\n\
            *When the base takes 240,000 damage:\n\
            **1 [[One Horn]] (600%) spawns.\n\
            *When the base takes 300,000 damage:\n\
            **1 [[Doge Dark]] (100%) spawns as the boss.\n\
            **5 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **2 [[Shadow Boxer K]]s (100%) spawn, delay 10 seconds<sup>300f</sup>.\n\
            **3 [[Shadow Boxer K]]s (100%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 360,000 damage:\n\
            **3 [[Doge Dark]]s (600%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 390,000 damage:\n\
            **1 [[Dagshund]] (300%) spawns.\n\
            *When the base takes 420,000 damage:\n\
            **3 [[Doge Dark]]s (600%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 450,000 damage:\n\
            **1 [[R.Ost]] (100%) spawns as the boss.\n\
            **5 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            *When the base takes 500,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (600%) spawns.\n\
            *When the base takes 550,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (600%) spawns.\n\
            *When the base takes 600,000 damage:\n\
            **1 [[The Face]] (1,000%) spawns as the boss.\n\
            **3 [[One Horn]]s (1,200%) spawn, delay 33.33 seconds<sup>1,000f</sup>.\n\
            **8 [[Doge Dark]]s (800%) spawn, delay 3.33~5 seconds<sup>100f~150f</sup>.\n\
            **5 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            *When the base takes 650,000 damage:\n\
            **1 [[Dagshund]] (400%) spawns.\n\
            *When the base takes 700,000 damage:\n\
            **1 [[Le'boin]] (1,800%) spawns.\n\
            *When the base takes 750,000 damage:\n\
            **1 [[Dagshund]] (400%) spawns.\n\
            *When the base takes 800,000 damage:\n\
            **1 [[Doge Dark]] (100%) spawns as the boss.\n\
            **5 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **2 [[Shadow Boxer K]]s (200%) spawn, delay 10 seconds<sup>300f</sup>.\n\
            **3 [[Shadow Boxer K]]s (200%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 850,000 damage:\n\
            **3 [[Doge Dark]]s (900%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 900,000 damage:\n\
            **1 [[One Horn]] (1,800%) spawns.\n\
            *When the base takes 950,000 damage:\n\
            **3 [[Doge Dark]]s (900%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 1,000,000 damage:\n\
            **1 [[Doge Dark]] (100%) spawns as the boss.\n\
            **7 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **2 [[R.Ost]]s (150%) spawn, delay 10 seconds<sup>300f</sup>.\n\
            *When the base takes 1,100,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (800%) spawns.\n\
            *When the base takes 1,150,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (800%) spawns.\n\
            *When the base takes 1,200,000 damage:\n\
            **1 [[The Face]] (1,200%) spawns as the boss.\n\
            **2 [[One Horn]]s (2,100%) spawn, delay 20 seconds<sup>600f</sup>.\n\
            **5 [[Doge Dark]]s (1,000%) spawn, delay 3.33~5 seconds<sup>100f~150f</sup>.\n\
            **7 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            *When the base takes 1,250,000 damage:\n\
            **1 [[Dagshund]] (500%) spawns.\n\
            *When the base takes 1,300,000 damage:\n\
            **1 [[Le'boin]] (2,100%) spawns.\n\
            *When the base takes 1,350,000 damage:\n\
            **1 [[Dagshund]] (500%) spawns.\n\
            *When the base takes 1,400,000 damage:\n\
            **1 [[Doge Dark]] (100%) spawns as the boss.\n\
            **7 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **2 [[Shadow Boxer K]]s (300%) spawn, delay 10 seconds<sup>300f</sup>.\n\
            **3 [[Shadow Boxer K]]s (300%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 1,450,000 damage:\n\
            **3 [[Doge Dark]]s (1,100%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 1,500,000 damage:\n\
            **1 [[One Horn]] (2,400%) spawns.\n\
            *When the base takes 1,550,000 damage:\n\
            **3 [[Doge Dark]]s (1,100%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 1,600,000 damage:\n\
            **1 [[Doge Dark]] (100%) spawns as the boss.\n\
            **7 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **2 [[R.Ost]]s (200%) spawn, delay 10 seconds<sup>300f</sup>.\n\
            *When the base takes 1,700,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (1,000%) spawns.\n\
            *When the base takes 1,750,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (1,000%) spawns.\n\
            *When the base takes 1,800,000 damage:\n\
            **1 [[The Face]] (1,500%) spawns as the boss.\n\
            **2 [[One Horn]]s (2,700%) spawn, delay 20 seconds<sup>600f</sup>.\n\
            **5 [[Doge Dark]]s (1,200%) spawn, delay 3.33~5 seconds<sup>100f~150f</sup>.\n\
            **7 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            *When the base takes 1,850,000 damage:\n\
            **1 [[Dagshund]] (600%) spawns.\n\
            *When the base takes 1,900,000 damage:\n\
            **1 [[Le'boin]] (2,700%) spawns.\n\
            *When the base takes 1,950,000 damage:\n\
            **1 [[Dagshund]] (600%) spawns.\n\
            *When the base takes 2,000,000 damage:\n\
            **1 [[Doge Dark]] (100%) spawns as the boss.\n\
            **9 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **2 [[Shadow Boxer K]]s (400%) spawn, delay 10 seconds<sup>300f</sup>.\n\
            **3 [[Shadow Boxer K]]s (400%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 2,050,000 damage:\n\
            **3 [[Doge Dark]]s (1,300%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 2,100,000 damage:\n\
            **1 [[One Horn]] (3,000%) spawns.\n\
            *When the base takes 2,150,000 damage:\n\
            **3 [[Doge Dark]]s (1,300%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 2,200,000 damage:\n\
            **1 [[Doge Dark]] (100%) spawns as the boss.\n\
            **9 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **2 [[R.Ost]]s (250%) spawn, delay 10 seconds<sup>300f</sup>.\n\
            *When the base takes 2,300,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (1,200%) spawns.\n\
            *When the base takes 2,350,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (1,200%) spawns.\n\
            *When the base takes 2,400,000 damage:\n\
            **1 [[The Face]] (1,800%) spawns as the boss.\n\
            **2 [[One Horn]]s (3,300%) spawn, delay 20 seconds<sup>600f</sup>.\n\
            **5 [[Doge Dark]]s (1,400%) spawn, delay 3.33~5 seconds<sup>100f~150f</sup>.\n\
            **9 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            *When the base takes 2,450,000 damage:\n\
            **1 [[Dagshund]] (700%) spawns.\n\
            *When the base takes 2,500,000 damage:\n\
            **1 [[Le'boin]] (3,300%) spawns.\n\
            *When the base takes 2,550,000 damage:\n\
            **1 [[Dagshund]] (700%) spawns.\n\
            *When the base takes 2,600,000 damage:\n\
            **1 [[Doge Dark]] (100%) spawns as the boss.\n\
            **9 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **2 [[Shadow Boxer K]]s (500%) spawn, delay 10 seconds<sup>300f</sup>.\n\
            **3 [[Shadow Boxer K]]s (500%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 2,650,000 damage:\n\
            **3 [[Doge Dark]]s (1,500%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 2,700,000 damage:\n\
            **1 [[One Horn]] (3,600%) spawns.\n\
            *When the base takes 2,750,000 damage:\n\
            **3 [[Doge Dark]]s (1,500%) spawn, delay 5~10 seconds<sup>150f~300f</sup>.\n\
            *When the base takes 2,800,000 damage:\n\
            **1 [[Doge Dark]] (100%) spawns as the boss.\n\
            **9 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~6.67 seconds<sup>100f~200f</sup>.\n\
            **2 [[R.Ost]]s (300%) spawn, delay 10 seconds<sup>300f</sup>.\n\
            *When the base takes 2,900,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (1,400%) spawns.\n\
            *When the base takes 2,950,000 damage:\n\
            **1 [[St. Pigge the 2nd]] (1,400%) spawns.\n\
            *When the base takes 3,000,000 damage:\n\
            **1 [[The Face]] (2,100%) spawns as the boss.\n\
            **2 [[One Horn]]s (3,900%) spawn, delay 20 seconds<sup>600f</sup>.\n\
            **5 [[Doge Dark]]s (1,600%) spawn, delay 3.33~5 seconds<sup>100f~150f</sup>.\n\
            **9 [[Fireworks Guys (High-Yield)]] spawn, delay 3.33~10 seconds<sup>100f~300f</sup>."
        );
    }
}
