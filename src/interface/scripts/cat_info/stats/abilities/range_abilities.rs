use super::{util::{get_ability_single, get_range_repr}, write_abilities::get_pure_abilities};
use crate::{
    game_data::cat::parsed::stats::form::{AttackHit, AttackHits, AttackRange, CatFormStats},
    interface::error_handler::InfallibleWrite,
};
use std::fmt::Write;

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
