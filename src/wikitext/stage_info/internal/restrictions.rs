//! Get the stage's restrictions.

use crate::{
    data::stage::{
        parsed::stage::{Restriction, RestrictionCrowns as Crowns, Stage},
        stage_option::charagroups::{CharaGroup, CharaGroupType},
    },
    wikitext::{data_files::cat_data::CAT_DATA, template_parameter::TemplateParameter},
};
use num_format::{Locale, WriteFormatted};
use std::{
    collections::HashSet,
    fmt::Write,
    num::{NonZero, NonZeroU8},
};

/// Sometimes I don't like Rust's type system.
const fn non_zero_u8(value: u8) -> NonZero<u8> {
    match NonZeroU8::new(value) {
        Some(v) => v,
        None => panic!("Value must be non-zero!"),
    }
}
/// Specials and rares only and only applies to 4-crown.
const FOUR_CROWN_DEFAULT_RESTRICTION: Restriction = Restriction {
    crowns_applied: Crowns::One(non_zero_u8(4)),
    rarity: NonZeroU8::new(0b000110),
    deploy_limit: None,
    rows: None,
    min_cost: None,
    max_cost: None,
    charagroup: None,
};

/// Get only rarities allowed.
fn get_rarity_restriction(rarity: NonZero<u8>) -> String {
    let rarities: Vec<&str> = (0..6)
        .filter_map(|i| {
            let rarity_bit = u8::from(rarity) & (1 << i);
            if rarity_bit == 0 {
                return None;
            };

            let rarity = match i {
                0 => "[[:Category:Normal Cats|Normal]]",
                1 => "[[:Category:Special Cats|Special]]",
                2 => "[[:Category:Rare Cats|Rare]]",
                3 => "[[:Category:Super Rare Cats|Super Rare]]",
                4 => "[[:Category:Uber Rare Cats|Uber Rare]]",
                5 => "[[:Category:Legend Rare Cats|Legend Rare]]",
                _ => unreachable!(),
            };
            Some(rarity)
        })
        .collect();

    let mut buf = "Rarity: Only ".to_string();
    if rarities.len() == 1 {
        buf.write_str(rarities[0]).unwrap();
    } else {
        let (last, first) = rarities.split_last().unwrap();
        let grouped = first.join(", ");
        buf.write_str(&grouped).unwrap();
        buf.write_str(" and ").unwrap();
        buf.write_str(last).unwrap();
    }
    buf
}

/// Get the restriction defined by the charagroup.
fn get_charagroup_restriction(group: &CharaGroup) -> String {
    // Alternatively, hardcode some of these like heartbeat catcademy and JRA
    // since they'll always be changing but will always have the same concept.
    let mut buf = "Unit Restriction: ".to_string();
    let mode = match group.group_type {
        CharaGroupType::OnlyUse => "Only",
        CharaGroupType::CannotUse => "Cannot use",
    };
    buf.write_str(mode).unwrap();
    buf.write_str(" ").unwrap();
    let groupunits: Vec<String> = group
        .units
        .iter()
        .map(|unit| CAT_DATA.get_cat_link(*unit))
        .collect();

    if groupunits.len() == 1 {
        buf.write_str(&groupunits[0]).unwrap();
    } else {
        let (last, first) = groupunits.split_last().unwrap();
        let grouped = first.join(", ");
        buf.write_str(&grouped).unwrap();
        buf.write_str(" and ").unwrap();
        buf.write_str(last).unwrap();
    }

    buf
}

/// Get a list of restrictions that a single [Restriction] object corresponds
/// to.
fn get_single_restriction(restriction: &Restriction) -> Vec<String> {
    let mut restrictions = vec![];

    if let Some(rarity) = restriction.rarity {
        let buf = get_rarity_restriction(rarity);
        restrictions.push(buf);
    }
    if let Some(limit) = restriction.deploy_limit {
        let mut buf = "".to_string();
        write!(buf, "Max # of Deployable Cats: {}", limit).unwrap();
        restrictions.push(buf);
    }
    if let Some(row) = restriction.rows {
        let mut buf = "".to_string();
        write!(buf, "Deploy from Row {} only", row).unwrap();
        restrictions.push(buf);
    }
    if let Some(min) = restriction.min_cost {
        let mut buf = "".to_string();
        buf.write_str("Cat Deploy Cost: Only ").unwrap();
        buf.write_formatted(&min, &Locale::en).unwrap();
        buf.write_str("¢ or more").unwrap();
        restrictions.push(buf);
    }
    if let Some(max) = restriction.max_cost {
        let mut buf = "".to_string();
        buf.write_str("Cat Deploy Cost: Only ").unwrap();
        buf.write_formatted(&max, &Locale::en).unwrap();
        buf.write_str("¢ or less").unwrap();
        restrictions.push(buf);
    }
    if let Some(group) = restriction.charagroup {
        let buf = get_charagroup_restriction(group);
        restrictions.push(buf)
    }

    restrictions
}

/// Helper function for [get_multi_restriction]. If `res_new` is already in
/// `restriction_crowns`, then add `crown` to that restriction crown's list.
/// Otherwise, create a new `restriction_crowns` entry and place `crown` in that
/// restriction crown's list.
fn add_restriction_or_crown(
    restriction_crowns: &mut Vec<(String, Vec<u8>)>,
    res_new: String,
    crown: u8,
) {
    if let Some((_, crowns)) = restriction_crowns
        .iter_mut()
        .find(|(res_already, _)| res_already == &res_new)
    {
        crowns.push(crown)
    } else {
        restriction_crowns.push((res_new, vec![crown]))
    }
}

/// Assert that the restriction hasn't been duplicated, which means that
/// `restriction_crowns.len()` is the amount of crowns the restriction applies
/// to.
// Is this necessary, probably not
fn assert_all_restrictions_unique(restriction_crowns: &[(String, Vec<u8>)]) {
    assert!(restriction_crowns.iter().all(|(_, crowns)| {
        let mut seen = HashSet::new();
        crowns.iter().all(|crown| seen.insert(crown))
    }))
}

/// Get restrictions when the stage has multiple restrictions being applied to
/// it.
fn get_multi_restriction(restrictions: &Vec<Restriction>, max_difficulty: u8) -> Vec<String> {
    let mut restriction_crowns: Vec<(String, Vec<u8>)> = vec![];
    for restriction in restrictions {
        let crown: u8 = match restriction.crowns_applied {
            Crowns::One(c) => c.into(),
            Crowns::All => panic!("Stage has multiple restrictions that do not apply equally!"),
        };
        for r in get_single_restriction(restriction) {
            add_restriction_or_crown(&mut restriction_crowns, r, crown);
        }
    }

    assert_all_restrictions_unique(&restriction_crowns);
    assert!(restriction_crowns.len() >= 2, "PONOS is bad at their job.");

    restriction_crowns
        .into_iter()
        .map(|(r, crowns)| match crowns.len() {
            x if x == max_difficulty as usize => r,
            1 => format!("{}-Crown: {}", crowns[0], r),
            _ => panic!("Restrictions don't apply to all stages!"),
        })
        .collect()
}

/// Get a list of stage restrictions if they exist.
fn get_restriction_list(stage: &Stage) -> Option<Vec<String>> {
    let restrictions = stage.restrictions.as_ref()?;
    if restrictions.is_empty() || restrictions == &[FOUR_CROWN_DEFAULT_RESTRICTION] {
        return None;
    }

    if restrictions.len() == 1
    // || (restrictions.len() == 2 && restrictions[1] == FOUR_CROWN_DEFAULT_RESTRICTION)
    {
        let restriction = &restrictions[0];
        if restriction.crowns_applied != Crowns::All
            && stage.crown_data.is_some()
            && (stage.crown_data.as_ref().unwrap().max_difficulty > non_zero_u8(1)
                || restriction.crowns_applied != Crowns::One(non_zero_u8(1)))
        // invalidate if either:
        // - stage has multiple crowns available and the restriction does not
        //   apply to all crowns
        // - stage has only 1 crown available and the restriction does not apply
        //   to that crown (i.e. crowns isn't equal to All and isn't equal to
        //   One(1))
        // I don't expect any stage to have these problems but it's better to be
        // safe than sorry (the Rust way!).
        {
            panic!("Unexpected crown error in stage: {stage:?}");
        }

        return Some(get_single_restriction(restriction));
    }

    let max_difficulty = u8::from(stage.crown_data.as_ref().unwrap().max_difficulty);
    assert_eq!(
        restrictions.len(),
        max_difficulty as usize,
        "Mismatch of amount of restrictions and amount of crowns! \
        Restrictions: {restrictions:?}"
    );
    Some(get_multi_restriction(restrictions, max_difficulty))
}

/// Get restrictions for Stage Info template (including no continues).
pub fn restrictions_info(stage: &Stage) -> Option<TemplateParameter> {
    const PARAM_NAME: &str = "restriction";

    let restrictions = get_restriction_list(stage);
    let r = match restrictions {
        None => {
            if !stage.is_no_continues {
                return None;
            }
            return Some(TemplateParameter::new(
                PARAM_NAME,
                "[[No Continues]]".to_string(),
            ));
        }
        Some(r) => r,
    };

    let mut buf = r.join("<br>\n");
    if stage.is_no_continues {
        buf.write_str("<br>\n[[No Continues]]").unwrap();
    }

    Some(TemplateParameter::new(PARAM_NAME, buf))
}

/// Get content of restrictions section.
pub fn restrictions_section(stage: &Stage) -> String {
    let restrictions = match get_restriction_list(stage) {
        None => return "".to_string(),
        Some(r) => r,
    };

    if restrictions.len() == 1 {
        return restrictions.into_iter().next().unwrap();
    }

    let mut buf = "".to_string();
    for restriction in restrictions {
        buf.write_str("*").unwrap();
        buf.write_str(&restriction).unwrap();
        buf.write_str("\n").unwrap();
    }
    buf.truncate(buf.len() - 1);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_restrictions() {
        let boxing_clever = Stage::new("s 50 1").unwrap();
        assert_eq!(boxing_clever.restrictions, None);
        assert_eq!(restrictions_info(&boxing_clever), None);
        assert_eq!(&restrictions_section(&boxing_clever), "");
    }

    #[test]
    fn no_continues() {
        let realm_of_carnage = Stage::new("s 117 0").unwrap();
        assert_eq!(realm_of_carnage.restrictions, None);
        assert_eq!(
            restrictions_info(&realm_of_carnage),
            Some(TemplateParameter::new(
                "restriction",
                "[[No Continues]]".to_string()
            ))
        );
        assert_eq!(&restrictions_section(&realm_of_carnage), "");
    }

    #[test]
    fn only_4_crown_restrictions() {
        let earthshaker = Stage::new("sol 0 0").unwrap();
        assert_eq!(
            earthshaker.restrictions.as_ref().unwrap(),
            &[FOUR_CROWN_DEFAULT_RESTRICTION]
        );
        assert_eq!(restrictions_info(&earthshaker), None);
        assert_eq!(&restrictions_section(&earthshaker), "");
    }

    #[test]
    fn restriction_rarity_1() {
        let sighter_star = Stage::new("cotc 3 24").unwrap();
        assert_eq!( restrictions_info (&sighter_star), Some(TemplateParameter::new( "restriction", "Rarity: Only [[:Category:Special Cats|Special]], [[:Category:Rare Cats|Rare]] and [[:Category:Super Rare Cats|Super Rare]]".to_string() )) );
        assert_eq!( &restrictions_section(&sighter_star), "Rarity: Only [[:Category:Special Cats|Special]], [[:Category:Rare Cats|Rare]] and [[:Category:Super Rare Cats|Super Rare]]" );
    }

    #[test]
    fn restriction_rarity_2() {
        let babies_first = Stage::new("s 375 0").unwrap();
        assert_eq!( restrictions_info (&babies_first), Some(TemplateParameter::new( "restriction", "Rarity: Only [[:Category:Normal Cats|Normal]] and [[:Category:Uber Rare Cats|Uber Rare]]<br>\n[[No Continues]]".to_string() )) );
        assert_eq!( &restrictions_section(&babies_first), "Rarity: Only [[:Category:Normal Cats|Normal]] and [[:Category:Uber Rare Cats|Uber Rare]]" );
    }

    #[test]
    fn restriction_rarity_3() {
        let somolon = Stage::new("cotc 3 37").unwrap();
        assert_eq!(
            restrictions_info(&somolon),
            Some(TemplateParameter::new(
                "restriction",
                "Rarity: Only [[:Category:Special Cats|Special]]".to_string()
            ))
        );
        assert_eq!(
            &restrictions_section(&somolon),
            "Rarity: Only [[:Category:Special Cats|Special]]"
        );
    }

    #[test]
    fn restriction_rarity_4() {
        let wahwah = Stage::new("s 158 0").unwrap();
        assert_eq!( restrictions_info (&wahwah), Some(TemplateParameter::new( "restriction", "Rarity: Only [[:Category:Normal Cats|Normal]], [[:Category:Uber Rare Cats|Uber Rare]] and [[:Category:Legend Rare Cats|Legend Rare]]<br>\n[[No Continues]]".to_string() )) );
        assert_eq!( &restrictions_section(&wahwah), "Rarity: Only [[:Category:Normal Cats|Normal]], [[:Category:Uber Rare Cats|Uber Rare]] and [[:Category:Legend Rare Cats|Legend Rare]]" );
    }

    #[test]
    fn restriction_deploy_limit() {
        let wrath_w_cyclone = Stage::new("s 176 0").unwrap();
        assert_eq!(
            restrictions_info(&wrath_w_cyclone),
            Some(TemplateParameter::new(
                "restriction",
                "Max # of Deployable Cats: 10".to_string()
            ))
        );
        assert_eq!(
            &restrictions_section(&wrath_w_cyclone),
            "Max # of Deployable Cats: 10"
        );
    }

    #[test]
    fn restriction_rows() {
        let uranus = Stage::new("cotc 2 7").unwrap();
        assert_eq!(
            restrictions_info(&uranus),
            Some(TemplateParameter::new(
                "restriction",
                "Deploy from Row 1 only".to_string()
            ))
        );
        assert_eq!(&restrictions_section(&uranus), "Deploy from Row 1 only");
    }

    #[test]
    fn restriction_min_cost_1() {
        let saturn = Stage::new("cotc 2 3").unwrap();
        assert_eq!(
            restrictions_info(&saturn),
            Some(TemplateParameter::new(
                "restriction",
                "Cat Deploy Cost: Only 300¢ or more".to_string()
            ))
        );
        assert_eq!(
            &restrictions_section(&saturn),
            "Cat Deploy Cost: Only 300¢ or more"
        );
    }

    #[test]
    fn restriction_min_cost_2() {
        let skelling = Stage::new("cotc 2 40").unwrap();
        assert_eq!(
            restrictions_info(&skelling),
            Some(TemplateParameter::new(
                "restriction",
                "Cat Deploy Cost: Only 1,200¢ or more".to_string()
            ))
        );
        assert_eq!(
            &restrictions_section(&skelling),
            "Cat Deploy Cost: Only 1,200¢ or more"
        );
    }

    #[test]
    fn restriction_max_cost_1() {
        let buutara = Stage::new("cotc 1 27").unwrap();
        assert_eq!(
            restrictions_info(&buutara),
            Some(TemplateParameter::new(
                "restriction",
                "Cat Deploy Cost: Only 1,200¢ or less".to_string()
            ))
        );
        assert_eq!(
            &restrictions_section(&buutara),
            "Cat Deploy Cost: Only 1,200¢ or less"
        );
    }

    #[test]
    fn restriction_max_cost_2() {
        let catseye_nebula = Stage::new("cotc 1 13").unwrap();
        assert_eq!(
            restrictions_info(&catseye_nebula),
            Some(TemplateParameter::new(
                "restriction",
                "Cat Deploy Cost: Only 4,000¢ or less".to_string()
            ))
        );
        assert_eq!(
            &restrictions_section(&catseye_nebula),
            "Cat Deploy Cost: Only 4,000¢ or less"
        );
    }

    #[test]
    fn restriction_only_cat() {
        let finale = Stage::new("c 209 0").unwrap();
        assert_eq!(
            restrictions_info(&finale),
            Some(TemplateParameter::new(
                "restriction",
                "Unit Restriction: Only [[Cat (Normal Cat)|Cat]]".to_string()
            ))
        );
        assert_eq!(
            &restrictions_section(&finale),
            "Unit Restriction: Only [[Cat (Normal Cat)|Cat]]"
        );
    }

    #[test]
    fn restriction_only_jra() {
        let final_race = Stage::new("c 179 0").unwrap();
        assert_eq!(
            restrictions_info(&final_race),
            Some(TemplateParameter::new(
                "restriction",
                "Unit Restriction: Only [[Cat Giraffe Modoki (Special Cat)|Cat Giraffe Modoki]], [[Catnip Tricky (Special Cat)|Catnip Tricky]] and [[Catnip Dragon (Special Cat)|Catnip Dragon]]".to_string()
            ))
        );
        assert_eq!(
            &restrictions_section(&final_race),
            "Unit Restriction: Only [[Cat Giraffe Modoki (Special Cat)|Cat Giraffe Modoki]], [[Catnip Tricky (Special Cat)|Catnip Tricky]] and [[Catnip Dragon (Special Cat)|Catnip Dragon]]"
        );
    }

    #[test]
    fn restriction_exclude_madoka() {
        let sorry = Stage::new("c 178 4").unwrap();
        assert_eq!(
            restrictions_info(&sorry),
            Some(TemplateParameter::new(
                "restriction",
                "Unit Restriction: Cannot use [[Homura Akemi (Uber Rare Cat)|Homura Akemi]] and [[Li'l Homura (Special Cat)|Li'l Homura]]<br>\n[[No Continues]]".to_string()
            ))
        );
        assert_eq!(
            &restrictions_section(&sorry),
            "Unit Restriction: Cannot use [[Homura Akemi (Uber Rare Cat)|Homura Akemi]] and [[Li'l Homura (Special Cat)|Li'l Homura]]"
        );
    }

    #[test]
    fn restriction_multiple_cotc() {
        let black_hole = Stage::new("cotc 2 46").unwrap();
        assert_eq!(
            restrictions_info(&black_hole),
            Some(TemplateParameter::new(
                "restriction",
                "Rarity: Only [[:Category:Special Cats|Special]], [[:Category:Rare Cats|Rare]], \
                [[:Category:Uber Rare Cats|Uber Rare]] and \
                [[:Category:Legend Rare Cats|Legend Rare]]<br>\n\
                Max # of Deployable Cats: 10"
                    .to_string()
            ))
        );
        assert_eq!(
            restrictions_section(&black_hole),
            "*Rarity: Only [[:Category:Special Cats|Special]], [[:Category:Rare Cats|Rare]], [[:Category:Uber Rare Cats|Uber Rare]] and [[:Category:Legend Rare Cats|Legend Rare]]\n\
            *Max # of Deployable Cats: 10"
        );
    }

    #[test]
    fn individual_crown_restrictions() {
        let feathered = Stage::new("c 86 0").unwrap();

        assert_eq!(
            restrictions_info(&feathered),
            Some(TemplateParameter::new(
                "restriction",
                "Rarity: Only [[:Category:Normal Cats|Normal]], [[:Category:Special Cats|Special]] and [[:Category:Rare Cats|Rare]]<br>\n\
                4-Crown: Max # of Deployable Cats: 10"
                    .to_string()
            )),
        );
        assert_eq!(
            restrictions_section(&feathered),
            "*Rarity: Only [[:Category:Normal Cats|Normal]], [[:Category:Special Cats|Special]] and [[:Category:Rare Cats|Rare]]\n\
            *4-Crown: Max # of Deployable Cats: 10"
        );
    }

    #[test]
    fn blank_some_restrictions() {
        let revenge_r_cyclone = Stage::new("s 169 1").unwrap();
        assert_eq!(revenge_r_cyclone.restrictions, Some(vec![]));
        assert_eq!(restrictions_info(&revenge_r_cyclone), None);
        assert_eq!(restrictions_section(&revenge_r_cyclone), "");
    }

    #[test]
    #[should_panic]
    fn test_assert_all_restrictions_unique() {
        let restrictions: &[(std::string::String, Vec<u8>)] = &[
            ("Rarity: Short lines".to_string(), [1, 1, 1, 1].to_vec()),
            ("Max # of Deployable Cats: 10".to_string(), [4].to_vec()),
        ];
        assert_all_restrictions_unique(restrictions);
    }
}
