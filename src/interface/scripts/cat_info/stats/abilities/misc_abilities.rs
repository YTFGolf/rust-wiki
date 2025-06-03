use super::util::{get_ability_single, get_range_repr};
use crate::{
    game_data::cat::{
        parsed::stats::form::{AttackHit, AttackHits, AttackRange, CatFormStats},
        raw::unitlevel::UnitLevelRaw,
    },
    interface::error_handler::InfallibleWrite,
    wikitext::number_utils::time_repr,
};
use num_format::{Locale, WriteFormatted};
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
pub fn get_range_ability(hits: &AttackHits) -> Option<String> {
    match hits {
        AttackHits::Single([hit1]) => {
            let mut hit = get_first_hit_range(hit1)?;
            hit += ")";
            Some(hit)
        }
        AttackHits::Double([hit1, hit2]) => {
            let mut hit = get_first_hit_range(hit1)?;

            if hit1.range == hit2.range {
                hit += ")";
                return Some(hit);
            }

            if !(hit2.range == AttackRange::Unchanged || hit1.range.has_same_type(&hit2.range)) {
                log::warn!("Hits 1 and 2 are of completely different types")
                // not needed tbh: either:
                // - Normal: panics anyway
                // - Unchanged: valid anyway
                // Therefore only difference is if first hit is LD/Omni and
                // other is the other, they get formatted in the same way so not
                // an issue.
            }

            write_hit_2(&mut hit, hit2);
            hit.write_char(')').infallible_write();
            Some(hit)
        }
        AttackHits::Triple([hit1, hit2, hit3]) => {
            let mut hit = get_first_hit_range(hit1)?;

            if hit1.range == hit2.range && hit1.range == hit3.range {
                hit += ")";
                return Some(hit);
            }

            if !hit2.range.has_same_type(&hit3.range) {
                log::warn!(
                    "Hits 2 and 3 are of incompatible types: {:?} vs {:?}",
                    hit2.range,
                    hit3.range
                );
            }
            if !(hit2.range == AttackRange::Unchanged || hit1.range.has_same_type(&hit2.range)) {
                log::warn!("Hits 1 and 2 are of completely different types")
                // not needed for same reasons as in double
            }

            write_hit_2(&mut hit, hit2);
            write_hit_3(&mut hit, hit3);
            hit.write_char(')').infallible_write();
            Some(hit)
        }
    }
}

fn write_hit(buf: &mut String, hit: &AttackHit, scaling: &UnitLevelRaw, level: u8) {
    let hit_dmg_at_level = &scaling.get_stat_at_level(hit.damage, level);
    buf.write_formatted(hit_dmg_at_level, &Locale::en)
        .infallible_write();
    buf.write_str(" at ").infallible_write();
    let (fore_f, fore_s) = time_repr(hit.foreswing.into());
    write!(buf, "{fore_f}f <sup>{fore_s}s</sup>").infallible_write();
}

pub fn get_multihit_ability(
    stats: &CatFormStats,
    scaling: &UnitLevelRaw,
    level: u8,
) -> Option<String> {
    match &stats.attack.hits {
        AttackHits::Single(_) => None,
        AttackHits::Double([h1, h2]) => {
            let mut buf = "[[Special Abilities#Multi-Hit|Multi-Hit]] (".to_string();

            write_hit(&mut buf, h1, scaling, level);
            buf.write_str(", ").infallible_write();
            write_hit(&mut buf, h2, scaling, level);
            buf.write_str(")").infallible_write();

            Some(buf)
        }
        AttackHits::Triple([h1, h2, h3]) => {
            let mut buf = "[[Special Abilities#Multi-Hit|Multi-Hit]] (".to_string();

            write_hit(&mut buf, h1, scaling, level);
            buf.write_str(", ").infallible_write();
            write_hit(&mut buf, h2, scaling, level);
            buf.write_str(", ").infallible_write();
            write_hit(&mut buf, h3, scaling, level);
            buf.write_str(")").infallible_write();

            Some(buf)
        }
    }
}
