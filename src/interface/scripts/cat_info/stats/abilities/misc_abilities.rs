//! Miscellaneous abilities (LD/Omni/Multihit).

use super::util::{get_ability_single, get_range_repr};
use crate::{
    game_data::cat::{
        parsed::stats::form::{AttackHit, AttackHits, AttackRange},
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

/// Get range ability text (ld/omni).
pub fn get_range_ability(hits: &AttackHits) -> Vec<String> {
    match hits {
        AttackHits::Single([hit1]) => get_range_ability_same(hit1).into_iter().collect(),
        AttackHits::Double([hit1, hit2]) => {
            if hit2.range == AttackRange::Unchanged || hit2.range == hit1.range {
                get_range_ability_same(hit1).into_iter().collect()
            } else {
                get_range_ability_different(hits)
            }
        }
        AttackHits::Triple([hit1, hit2, hit3]) => {
            if hit2.range == AttackRange::Unchanged
                || hit3.range == AttackRange::Unchanged
                || hit2.range == hit1.range
                || hit3.range == hit1.range
            {
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

/// Get string representation of multihit ability.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TEST_CONFIG, game_data::cat::parsed::cat::Cat};

    const MULTI_HIT_INTRO: &str =
        "{{AbilityIcon|Multi-Hit}} [[Special Abilities#Multi-Hit|Multi-Hit]]";
    const LD_INTRO: &str =
        "{{AbilityIcon|Long Distance}} [[Special Abilities#Long Distance|Long Distance]]";
    const OMNI_INTRO: &str =
        "{{AbilityIcon|Omni Strike}} [[Special Abilities#Omni Strike|Omni Strike]]";

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
        assert_eq!(get_range_ability(&form.attack.hits), Vec::<String>::new());
    }

    #[test]
    fn multi_no_ld() {
        let bahamut = get_stats(25);
        let form = &bahamut.forms.stats[2];

        let mh = "85,000 at 5f <sup>0.17s</sup>, 3,400 at 10f <sup>0.33s</sup>, 5,100 at 20f <sup>0.67s</sup>";
        assert_eq!(
            get_multihit_ability(&form.attack.hits, &bahamut.unitlevel, 30),
            Some(format!("{MULTI_HIT_INTRO} ({mh})"))
        );
        assert_eq!(get_range_ability(&form.attack.hits), Vec::<String>::new());
    }

    #[test]
    fn ld_no_multi() {
        let cyberpunk = get_stats(35);
        let form = &cyberpunk.forms.stats[2];
        assert_eq!(
            get_multihit_ability(&form.attack.hits, &cyberpunk.unitlevel, 30),
            None
        );
        assert_eq!(
            get_range_ability(&form.attack.hits),
            vec![format!("{LD_INTRO} (Effective range: 800~1,200)")]
        );
    }

    #[test]
    fn multi_and_ld() {
        let kasli = get_stats(529);
        let form = &kasli.forms.stats[0];

        let mh = "3,400 at 48f <sup>1.6s</sup>, 6,800 at 67f <sup>2.23s</sup>";
        assert_eq!(
            get_multihit_ability(&form.attack.hits, &kasli.unitlevel, 30),
            Some(format!("{MULTI_HIT_INTRO} ({mh})"))
        );
        assert_eq!(
            get_range_ability(&form.attack.hits),
            vec![format!("{LD_INTRO} (Effective range: 200~500)")]
        );
    }

    #[test]
    fn multi_and_ld_multiple() {
        let phonoa = get_stats(690);
        let form = &phonoa.forms.stats[1];

        let mh = "10,200 at 70f <sup>2.33s</sup>, 10,200 at 80f <sup>2.67s</sup>, 10,200 at 90f <sup>3s</sup>";
        let ld = "Effective range: 250~600 on 1st hit, 450~800 on 2nd hit, 590~1,000 on 3rd hit";
        assert_eq!(
            get_multihit_ability(&form.attack.hits, &phonoa.unitlevel, 30),
            Some(format!("{MULTI_HIT_INTRO} ({mh})"))
        );
        assert_eq!(
            get_range_ability(&form.attack.hits),
            vec![format!("{LD_INTRO} ({ld})")]
        );
    }

    #[test]
    fn ld_then_omni() {
        let carrowsell = get_stats(674);
        let form = &carrowsell.forms.stats[2];

        let mh = "17,000 at 85f <sup>2.83s</sup>, 17,000 at 89f <sup>2.97s</sup>, 17,000 at 93f <sup>3.1s</sup>";
        let ld = "Effective range: 1~401 on 1st hit";
        let omni = "Effective range: -35~435 on 2nd hit, -70~470 on 3rd hit";
        assert_eq!(
            get_multihit_ability(&form.attack.hits, &carrowsell.unitlevel, 30),
            Some(format!("{MULTI_HIT_INTRO} ({mh})"))
        );
        assert_eq!(
            get_range_ability(&form.attack.hits),
            vec![
                format!("{LD_INTRO} ({ld})"),
                format!("{OMNI_INTRO} ({omni})"),
            ]
        );
    }

    #[test]
    fn omni_and_ld() {
        let hanasaka = get_stats(769);
        let form = &hanasaka.forms.stats[1];

        let mh = "6,970 at 30f <sup>1s</sup>, 6,970 at 50f <sup>1.67s</sup>, 20,060 at 100f <sup>3.33s</sup>";
        let ld = "Effective range: -230~0 on 2nd hit";
        let omni = "Effective range: 0~230 on 1st hit, -230~230 on 3rd hit";
        assert_eq!(
            get_multihit_ability(&form.attack.hits, &hanasaka.unitlevel, 30),
            Some(format!("{MULTI_HIT_INTRO} ({mh})"))
        );
        assert_eq!(
            get_range_ability(&form.attack.hits),
            vec![
                format!("{LD_INTRO} ({ld})"),
                format!("{OMNI_INTRO} ({omni})"),
            ]
        );
    }

    #[test]
    fn ld_should_be_unchanged() {
        let dynasaurus = get_stats(763);
        // 3 hits with manually inputted range, but all ranges are 150~700 so
        // should behave the same as if the ranges were unchanged
        let form = &dynasaurus.forms.stats[0];

        let mh = "11,900 at 55f <sup>1.83s</sup>, 11,900 at 65f <sup>2.17s</sup>, 11,900 at 75f <sup>2.5s</sup>";
        assert_eq!(
            get_multihit_ability(&form.attack.hits, &dynasaurus.unitlevel, 30),
            Some(format!("{MULTI_HIT_INTRO} ({mh})"))
        );
        assert_eq!(
            get_range_ability(&form.attack.hits),
            vec![format!("{LD_INTRO} (Effective range: 150~700)"),]
        );
    }
}
