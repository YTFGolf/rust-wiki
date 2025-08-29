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

fn get_range_ability_different(hits: &AttackHits) -> Vec<String> {
    let mut ranges = vec![];

    let mut ld_buf = vec![];
    let mut omni_buf = vec![];

    let mut iter = hits.iter().enumerate();
    let (i, hit1) = iter.next().expect("will always be at least length 1");

    match hit1.range {
        AttackRange::Normal => (),
        AttackRange::Unchanged => unreachable!(),
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
            AttackRange::Unchanged => unreachable!(),
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

fn get_range_ability_same(hit1: &AttackHit) -> Option<String> {
    match hit1.range {
        AttackRange::Normal => None,
        AttackRange::Unchanged => panic!("Hit 1 is unchanged!"),
        AttackRange::LD { base, distance } => {
            let abil = "Long Distance";
            let t = format!(
                "{{{{AbilityIcon|{abil}}}}} {link} (Effective range: {range})",
                range = get_range_repr(base, base + distance),
                link = get_ability_single(abil)
            );
            Some(t)
        }
        AttackRange::Omni { base, distance } => {
            let abil = "Omni Strike";
            let t = format!(
                "{{{{AbilityIcon|{abil}}}}} {link} (Effective range: {range})",
                range = get_range_repr(base + distance, base),
                // distance is negative if is omni
                link = get_ability_single(abil)
            );
            Some(t)
        }
    }
}

pub fn get_range_ability(hits: &AttackHits) -> Vec<String> {
    match hits {
        AttackHits::Single([hit1]) => get_range_ability_same(hit1).into_iter().collect(),
        AttackHits::Double([hit1, hit2]) => {
            if hit2.range == AttackRange::Unchanged {
                get_range_ability_same(hit1).into_iter().collect()
            } else {
                get_range_ability_different(hits)
            }
        }
        AttackHits::Triple([hit1, hit2, hit3]) => {
            if hit2.range == AttackRange::Unchanged || hit3.range == AttackRange::Unchanged {
                assert_eq!(hit2.range, hit3.range);
                get_range_ability_same(hit1).into_iter().collect()
            } else {
                get_range_ability_different(hits)
            }
        }
    }
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
    hits: &AttackHits,
    scaling: &UnitLevelRaw,
    level: u8,
) -> Option<String> {
    match hits {
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

// TODO need to do a load of tests
// cat, Bahamut, Cyberpunk, Kasli (first form), Phonoa, Carrowsell, Hanasaka

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TEST_CONFIG, game_data::cat::parsed::cat::Cat};

    fn get_stats(id: u32) -> Cat {
        Cat::from_wiki_id(id, &TEST_CONFIG.version).unwrap()
    }

    #[test]
    fn basic() {
        let cat = get_stats(0);
        let form = &cat.forms.stats[2];
        assert_eq!(
            get_multihit_ability(&form.attack.hits, &cat.unitlevel, 30),
            None
        );
    }
}
