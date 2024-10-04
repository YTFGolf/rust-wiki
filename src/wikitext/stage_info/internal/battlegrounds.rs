use crate::{
    data::stage::parsed::{
        stage::Stage,
        stage_enemy::{BossType, EnemyAmount, StageEnemy},
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
    todo!()
}

/// Matcher string for all enemy bases that should begin with "is a*n*" instead
/// of "is a".
const AN_ENEMY_MATCHER: &str = r"^([AEIOU]|11|18|8)";

// just do edge cases described in The Battle Cats Wiki:Stage Structure
// Page/Battlegrounds

fn write_single_enemy(
    buf: &mut String,
    enemy: &StageEnemy,
    is_base_hit: bool,
    show_magnification: bool,
) {
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
        return;
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

    let is_instant_spawn = (enemy.start_frame == 0 || (is_base_hit && !enemy.enforce_start_frame));
    if !is_instant_spawn {
        buf.write_str(" after ").unwrap();
        write_enemy_delay(buf, enemy);
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
        write_enemy_delay(buf, enemy);
    }

    buf.write_char('.').unwrap();
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
