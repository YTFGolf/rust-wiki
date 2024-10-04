use crate::{
    data::stage::{
        parsed::{
            stage::Stage,
            stage_enemy::{BossType, EnemyAmount, StageEnemy},
        },
        stage_metadata::consts::StageTypeEnum as T,
    },
    wikitext::{data_files::enemy_data::ENEMY_DATA, wiki_utils::extract_name},
};
use either::Either::{Left, Right};
use num_format::{Locale, WriteFormatted};
use regex::Regex;
use std::{fmt::Write, num::NonZeroU32};

pub fn battlegrounds(stage: &Stage) -> String {
    // Go through all enemies
    // sort into percentages (if >100 then goes in 100)
    // sort all percentages
    let is_dojo = match stage.meta.type_enum {
        T::Dojo | T::RankingDojo => true,
        _ => false,
    };

    let is_default_spawn = match is_dojo {
        false => |enemy: &StageEnemy| enemy.base_hp >= 100,
        true => |enemy: &StageEnemy| enemy.base_hp == 0,
    };

    let mut default_spawn: Vec<&StageEnemy> = vec![];
    let mut other_spawn: Vec<(u32, Vec<&StageEnemy>)> = vec![];
    for enemy in stage.enemies.iter() {
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
            other_spawn.push((enemy.base_hp, vec![enemy]))
        }
    }
    let order_function: fn(
        &(u32, Vec<&StageEnemy>),
        &(u32, Vec<&StageEnemy>),
    ) -> std::cmp::Ordering = match is_dojo {
        false => |a, b| b.0.cmp(&a.0),
        true => |a, b| a.0.cmp(&b.0),
    };
    other_spawn.sort_by(order_function);

    // TODO sort individual lists
    // TODO remove 0 when not dojo
    // TODO assign duplicates
    // ignore (i.e. == 0 for non-dojo and false for dojo)
    // message

    // this is not an abstraction, this is a convenience. having a bool here
    // only works because I always know it's a bool
    fn stringify_enemy_list(enemies: Vec<&StageEnemy>, is_base_hit: bool) -> String {
        enemies
            .iter()
            .filter_map(|e| {
                if e.id == 21 && e.start_frame == 27_000 && e.boss_type == BossType::None {
                    None
                } else {
                    Some(get_single_enemy_line(e, is_base_hit, false))
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    let mut buf = "".to_string();
    if stage.is_base_indestructible {
        buf += "*The enemy base is indestructible until the boss is defeated.\n"
    }
    buf += &stringify_enemy_list(default_spawn, false);

    // TODO Dojo
    for other in other_spawn {
        buf += &format!("\n*When the base reaches {hp}% HP:\n", hp = other.0);
        buf += &stringify_enemy_list(other.1, true);
    }

    buf
}

/// Matcher string for all enemy bases that should begin with "is a*n*" instead
/// of "is a".
const AN_ENEMY_MATCHER: &str = r"^([AEIOU]|11|18|8)";

// just do edge cases described in The Battle Cats Wiki:Stage Structure
// Page/Battlegrounds

fn get_single_enemy_line(
    enemy: &StageEnemy,
    is_base_hit: bool,
    show_magnification: bool,
) -> String {
    let mut buf = "".to_string();

    if is_base_hit {
        buf.write_str("**").unwrap();
    } else {
        buf.write_str("*").unwrap();
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
    };

    match enemy.amount {
        EnemyAmount::Infinite => buf.write_str("Infinite").unwrap(),
        EnemyAmount::Limit(n) => {
            let _ = buf.write_formatted(&n, &Locale::en).unwrap();
        }
    };

    let is_single_enemy: bool = enemy.amount == EnemyAmount::Limit(NonZeroU32::new(1).unwrap());
    if is_single_enemy {
        write!(buf, " {}", ENEMY_DATA.get_names(enemy.id).name).unwrap();
    } else {
        write!(buf, " {}", ENEMY_DATA.get_names(enemy.id).plural).unwrap();
    }
    if show_magnification {
        buf.write_str(" (").unwrap();
        match enemy.magnification {
            Left(mag) => {
                buf.write_formatted(&mag, &Locale::en).unwrap();
                buf.write_str("%)").unwrap()
            }
            Right((hp, ap)) => {
                buf.write_formatted(&hp, &Locale::en).unwrap();
                buf.write_str("% HP, ").unwrap();
                buf.write_formatted(&ap, &Locale::en).unwrap();
                buf.write_str("% AP)").unwrap();
            }
        }
    }

    if is_single_enemy {
        buf.write_str(" spawns").unwrap();
    } else {
        buf.write_str(" spawn").unwrap();
    }

    if enemy.boss_type != BossType::None {
        match is_single_enemy {
            true => buf.write_str(" as the boss").unwrap(),
            false => buf.write_str(" as bosses").unwrap(),
        }
    }

    let is_instant_spawn = enemy.start_frame == 0 || (is_base_hit && !enemy.enforce_start_frame);
    if !is_instant_spawn {
        buf.write_str(" after ").unwrap();
        write_single_spawn_s(&mut buf, enemy.start_frame);
        if enemy.start_frame == 1 {
            buf.write_str(" second<sup>").unwrap();
        } else {
            buf.write_str(" seconds<sup>").unwrap();
        }
        buf.write_formatted(&enemy.start_frame, &Locale::en)
            .unwrap();
        buf.write_str("f</sup>").unwrap();
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

    buf.write_char('.').unwrap();

    buf
}

fn write_single_spawn_s(buf: &mut String, time_f: u32) {
    let respawn_s = f64::from(time_f) / 30.0;
    assert!(respawn_s < 1_000.0, "Spawn time is above 1,000 seconds!");
    let precision = if time_f % 30 == 0 {
        0
    } else if time_f % 3 == 0 {
        1
    } else {
        2
    };
    write!(buf, "{:.1$}", respawn_s, precision).unwrap();
}

fn write_enemy_delay(buf: &mut String, enemy: &StageEnemy) {
    buf.write_str(", delay ").unwrap();

    write_single_spawn_s(buf, enemy.respawn_time.0);
    if enemy.respawn_time.1 > enemy.respawn_time.0 {
        buf.write_char('~').unwrap();
        write_single_spawn_s(buf, enemy.respawn_time.1);
    }

    if enemy.respawn_time == (1, 1) {
        buf.write_str(" second").unwrap();
    } else {
        buf.write_str(" seconds").unwrap();
    }

    buf.write_str("<sup>").unwrap();
    buf.write_formatted(&enemy.respawn_time.0, &Locale::en)
        .unwrap();
    buf.write_char('f').unwrap();
    if enemy.respawn_time.1 > enemy.respawn_time.0 {
        buf.write_char('~').unwrap();
        buf.write_formatted(&enemy.respawn_time.1, &Locale::en)
            .unwrap();
        buf.write_char('f').unwrap();
    }
    buf.write_str("</sup>").unwrap();
}
