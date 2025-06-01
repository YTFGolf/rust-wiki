use super::write_abilities::get_pure_abilities;
use crate::{
    game_data::cat::parsed::stats::form::{AttackHit, AttackHits, AttackRange, CatFormStats},
    interface::error_handler::InfallibleWrite,
    wikitext::number_utils::{plural_f, time_repr},
};
use num_format::{Locale, WriteFormatted};
use std::fmt::Write;

pub fn get_duration_repr(duration: u32) -> String {
    let (dur_f, dur_s) = time_repr(duration);
    format!(
        "{dur_f}f <sub>{dur_s} {seconds}</sub>",
        seconds = plural_f(duration, "second", "seconds")
    )
}

pub fn get_ability(link: &str, display: &str) -> String {
    format!("[[Special Abilities#{link}|{display}]]")
}

pub fn get_ability_single(link_display: &str) -> String {
    format!("[[Special Abilities#{link_display}|{link_display}]]")
}

pub fn get_enemy_category(link: &str, display: &str) -> String {
    format!("[[:Category:{link} Enemies|{display}]]")
}

pub fn get_range_repr(min: i16, max: i16) -> String {
    let mut buf = String::new();
    buf.write_formatted(&min, &Locale::en).infallible_write();
    if min == max {
        return buf;
    }

    buf.write_char('~').infallible_write();
    buf.write_formatted(&max, &Locale::en).infallible_write();

    buf
}

/// Won't close the set of brackets.
fn get_first_hit_range(hit1: &AttackHit) -> Option<String> {
    match hit1.range {
        AttackRange::Normal => None,
        AttackRange::LD { base, distance } => Some(format!(
            "{ld} (Effective range: {range}",
            ld = get_ability_single("Long Distance"),
            range = get_range_repr(base, base + distance)
        )),
        AttackRange::Omni { base, distance } => Some(format!(
            "{omni} (Effective range: {range}",
            omni = get_ability_single("Omni Strike"),
            range = get_range_repr(base + distance, base) // distance is negative if is omni
        )),
        AttackRange::Unchanged => None,
    }
}

fn write_hit_2(buf: &mut String, hit2: &AttackHit) {
    match hit2.range {
        AttackRange::Normal => unreachable!(),
        AttackRange::Unchanged => (),
        AttackRange::LD { base, distance } => write!(
            buf,
            " on 1st hit, {range} on 2nd hit",
            range = get_range_repr(base, base + distance)
        )
        .infallible_write(),
        AttackRange::Omni { base, distance } => write!(
            buf,
            " on 1st hit, {range} on 2nd hit",
            range = get_range_repr(base + distance, base)
        )
        .infallible_write(),
    }
}

fn write_hit_3(buf: &mut String, hit3: &AttackHit) {
    match hit3.range {
        AttackRange::Normal => unreachable!(),
        AttackRange::Unchanged => (),
        AttackRange::LD { base, distance } => write!(
            buf,
            ", {range} on 3rd hit",
            range = get_range_repr(base, base + distance)
        )
        .infallible_write(),
        AttackRange::Omni { base, distance } => write!(
            buf,
            ", {range} on 3rd hit",
            range = get_range_repr(base + distance, base)
        )
        .infallible_write(),
    }
}

/// Get LD/Omni ability.
fn get_range_ability(hits: &AttackHits) -> Option<String> {
    match hits {
        AttackHits::Single([hit1]) => {
            let mut hit = get_first_hit_range(hit1)?;
            hit += ")";
            Some(hit)
        }
        AttackHits::Double([hit1, hit2]) => {
            let mut hit = get_first_hit_range(hit1)?;

            if !(hit2.range == AttackRange::Unchanged || hit1.range.has_same_type(&hit2.range)) {
                unimplemented!("Hits 1 and 2 are of completely different types")
            }

            write_hit_2(&mut hit, hit2);
            hit.write_char(')').infallible_write();
            Some(hit)
        }
        AttackHits::Triple([hit1, hit2, hit3]) => {
            let mut hit = get_first_hit_range(hit1)?;

            if !hit2.range.has_same_type(&hit3.range) {
                unimplemented!("Hits 2 and 3 are of incompatible types")
            }
            if !(hit2.range == AttackRange::Unchanged || hit1.range.has_same_type(&hit2.range)) {
                unimplemented!("Hits 1 and 2 are of completely different types")
            }

            write_hit_2(&mut hit, hit2);
            write_hit_3(&mut hit, hit3);
            hit.write_char(')').infallible_write();
            Some(hit)
        }
    }
}

/// Includes LD, Omni and any abilities, but does not include multhit.
pub fn get_abilities(abilities: &mut Vec<String>, stats: &CatFormStats) {
    abilities.extend(get_range_ability(&stats.attack.hits));
    abilities.extend(get_pure_abilities(stats));
}
