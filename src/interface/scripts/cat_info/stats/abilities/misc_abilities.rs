use super::util::{get_ability_single, get_range_repr};
use crate::{
    game_data::cat::{
        parsed::stats::form::{AttackHit, AttackHits, AttackRange, CatFormStats},
        raw::unitlevel::UnitLevelRaw,
    },
    interface::error_handler::InfallibleWrite,
    wikitext::{number_utils::time_repr, text_utils::get_ordinal},
};
use num_format::{Locale, WriteFormatted};
use std::fmt::Write;

/*
Need to do a load of tests

Top-level assert: hit 2 is unchanged iff hit 3 is unchanged

If normal: None
If unchanged: same as hit 1 (panic if found on hit 1)
If LD: add to ld buffer
If Omni: add to omni buffer

Store hit1
If normal store None
If LD/Omni then is Some(thing)
If unchanged panic

Go to next hit
If normal ensure hit1 is normal
If unchanged then do nothing (top-level assert checks that unchanged is
invariant)
If omni add to omni buffer
If LD add to LD buffer
*/

fn range_ability_text(hits_buf: Vec<(usize, String)>, abil: &str) -> Option<String> {
    let mut iter = hits_buf.into_iter();
    let f = iter.next()?;

    let mut buf = format!(
        "{{{{AbilityIcon|{abil}}}}} {link} (Effective range: ",
        link = get_ability_single(abil)
    );

    write!(
        buf,
        "{range} on {nth} hit",
        range = f.1,
        nth = get_ordinal(f.0 as u32)
    )
    .infallible_write();

    while let Some(f) = iter.next() {
        write!(
            buf,
            ", {range} on {nth} hit",
            range = f.1,
            nth = get_ordinal(f.0 as u32)
        )
        .infallible_write();
    }

    buf += ")";

    Some(buf)
}

pub fn get_range_ability(hits: &AttackHits) -> Vec<String> {
    match hits {
        AttackHits::Triple([.., hit2, hit3]) => {
            if hit2.range == AttackRange::Unchanged || hit3.range == AttackRange::Unchanged {
                assert_eq!(hit2.range, hit3.range);
            }
        }
        _ => (),
    }
    // just check that hit 2 is unchanged iff hit 3 is unchanged/non-existent

    let mut ranges = vec![];

    let mut ld_buf = vec![];
    let mut omni_buf = vec![];

    let mut iter = hits.iter().enumerate();
    let (i, hit1) = iter.next().expect("will always be at least length 1");

    match hit1.range {
        AttackRange::Normal => (),
        AttackRange::Unchanged => panic!("Hit 1 is unchanged!"),
        AttackRange::LD { base, distance } => {
            ld_buf.push((i + 1, get_range_repr(base, base + distance)))
        }
        AttackRange::Omni { base, distance } => {
            omni_buf.push((i + 1, get_range_repr(base + distance, base)))
            // distance is negative if is omni
        }
    }

    while let Some((i, hit)) = iter.next() {
        match hit.range {
            AttackRange::Normal => assert_eq!(hit.range, hit1.range),
            AttackRange::Unchanged => (),
            // no need to act when unchanged as the top-level assert statement
            // checks that hit 2 and hit 3 must have same range (or that hit 3
            // doesn't exist)
            AttackRange::LD { base, distance } => {
                ld_buf.push((i + 1, get_range_repr(base, base + distance)))
            }
            AttackRange::Omni { base, distance } => {
                omni_buf.push((i + 1, get_range_repr(base + distance, base)))
                // distance is negative if is omni
            }
        }
    }

    ranges.extend(range_ability_text(ld_buf, "Long Distance"));
    ranges.extend(range_ability_text(omni_buf, "Omni Strike"));

    ranges
}

fn write_mh_hit(buf: &mut String, hit: &AttackHit, scaling: &UnitLevelRaw, level: u8) {
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
            let mut buf =
                "{{AbilityIcon|Multi-Hit}} [[Special Abilities#Multi-Hit|Multi-Hit]] (".to_string();

            write_mh_hit(&mut buf, h1, scaling, level);
            buf.write_str(", ").infallible_write();
            write_mh_hit(&mut buf, h2, scaling, level);
            buf.write_str(")").infallible_write();

            Some(buf)
        }
        AttackHits::Triple([h1, h2, h3]) => {
            let mut buf =
                "{{AbilityIcon|Multi-Hit}} [[Special Abilities#Multi-Hit|Multi-Hit]] (".to_string();

            write_mh_hit(&mut buf, h1, scaling, level);
            buf.write_str(", ").infallible_write();
            write_mh_hit(&mut buf, h2, scaling, level);
            buf.write_str(", ").infallible_write();
            write_mh_hit(&mut buf, h3, scaling, level);
            buf.write_str(")").infallible_write();

            Some(buf)
        }
    }
}
