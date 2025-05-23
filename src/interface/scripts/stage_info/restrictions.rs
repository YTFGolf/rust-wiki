//! Get the stage's restrictions.

use crate::{
    data::{
        map::special_rules::{ContentsType, RuleType},
        stage::{
            parsed::stage::{Restriction, RestrictionCrowns as Crowns, RestrictionStages, Stage},
            raw::stage_option::charagroups::{CharaGroup, CharaGroupType},
        },
    },
    interface::error_handler::InfallibleWrite,
    wikitext::{data_files::cat_data::CAT_DATA, template::TemplateParameter},
};
use num_format::{Locale, WriteFormatted};
use std::{
    collections::HashSet,
    fmt::Write,
    num::{NonZero, NonZeroU8},
};

/// Convert `value` to a NonZero [`u8`].
const fn non_zero_u8(value: u8) -> NonZero<u8> {
    match NonZeroU8::new(value) {
        Some(v) => v,
        None => panic!("Value must be non-zero!"),
    }
    // TODO extract
}
/// Specials and rares only and only applies to 4-crown.
const FOUR_CROWN_DEFAULT_RESTRICTION: Restriction = Restriction {
    stages_applied: RestrictionStages::All,
    crowns_applied: Crowns::One(non_zero_u8(4)),
    rarity: NonZeroU8::new(0b00_0110),
    deploy_limit: None,
    rows: None,
    min_cost: None,
    max_cost: None,
    charagroup: None,
};

/// Get `"Rarity: Only ..."` restriction.
fn get_rarity_restriction(rarity: NonZero<u8>) -> String {
    let rarities: Vec<&str> = (0..6)
        .filter_map(|i| {
            let rarity_bit = u8::from(rarity) & (1 << i);
            // e.g. for number 0b00_0001
            // if i = 0 this gets the 1 at the end of the number
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
        buf.write_str(rarities[0]).infallible_write();
    } else {
        let (last, first) = rarities.split_last().unwrap();
        let grouped = first.join(", ");
        buf.write_str(&grouped).infallible_write();
        buf.write_str(" and ").infallible_write();
        buf.write_str(last).infallible_write();
    }
    buf
}

/// Get the restriction defined by the charagroup (i.e. the can only use or
/// cannot use certain units).
fn get_charagroup_restriction(group: &CharaGroup) -> String {
    // Alternatively, hardcode some of these like heartbeat catcademy and JRA
    // since they'll always be changing but will always have the same concept.
    let mut buf = "Unit Restriction: ".to_string();
    let mode = match group.group_type {
        CharaGroupType::OnlyUse => "Only",
        CharaGroupType::CannotUse => "Cannot use",
    };
    buf.write_str(mode).infallible_write();
    buf.write_str(" ").infallible_write();
    let groupunits: Vec<String> = group
        .units
        .iter()
        .map(|unit| CAT_DATA.get_cat_link(*unit))
        .collect();

    if groupunits.len() == 1 {
        buf.write_str(&groupunits[0]).infallible_write();
    } else {
        let (last, first) = groupunits.split_last().unwrap();
        let grouped = first.join(", ");
        buf.write_str(&grouped).infallible_write();
        buf.write_str(" and ").infallible_write();
        buf.write_str(last).infallible_write();
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
        let buf = format!("Max # of Deployable Cats: {limit}");
        restrictions.push(buf);
    }
    if let Some(row) = restriction.rows {
        let buf = format!("Deploy from Row {row} only");
        restrictions.push(buf);
    }
    if let Some(min) = restriction.min_cost {
        let mut buf = String::new();
        buf.write_str("Cat Deploy Cost: Only ").infallible_write();
        buf.write_formatted(&min, &Locale::en).infallible_write();
        buf.write_str("¢ or more").infallible_write();
        restrictions.push(buf);
    }
    if let Some(max) = restriction.max_cost {
        let mut buf = String::new();
        buf.write_str("Cat Deploy Cost: Only ").infallible_write();
        buf.write_formatted(&max, &Locale::en).infallible_write();
        buf.write_str("¢ or less").infallible_write();
        restrictions.push(buf);
    }
    if let Some(group) = &restriction.charagroup {
        let buf = get_charagroup_restriction(group);
        restrictions.push(buf);
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
        if !crowns.contains(&crown) {
            crowns.push(crown);
        }
    } else {
        restriction_crowns.push((res_new, vec![crown]));
    }
}

/// Assert that no numbers are duplicated in any of the nested u8 vecs.
fn assert_all_restrictions_unique(restriction_crowns: &[(String, Vec<u8>)]) {
    for (_, crowns) in restriction_crowns {
        let mut seen = HashSet::new();
        for crown in crowns {
            assert!(
                seen.insert(crown),
                "Crown {crown} is duplicated in vec {crowns:?}."
            );
        }
    }
}

/// Get restrictions when `restrictions.len()` is greater than 1.
fn get_multi_restriction(restrictions: &Vec<Restriction>, max_difficulty: u8) -> Vec<String> {
    let mut restriction_crowns: Vec<(String, Vec<u8>)> = vec![];
    for restriction in restrictions {
        let crown: u8 = match restriction.crowns_applied {
            Crowns::One(c) => c.into(),
            Crowns::All => panic!("Stage has multiple restrictions that do not apply equally!"),
            // i.e. restriction that applies to all crowns that isn't the only
            // restriction on the stage
            // not sure this error message is the best but it's better to not
            // deal with this case if I don't have to
        };
        for r in get_single_restriction(restriction) {
            add_restriction_or_crown(&mut restriction_crowns, r, crown);
        }
    }

    assert_all_restrictions_unique(&restriction_crowns);

    const FOUR_CROWN_DEFAULT: &str =
        "4-Crown: Rarity: Only [[:Category:Special Cats|Special]] and [[:Category:Rare Cats|Rare]]";
    restriction_crowns
        .into_iter()
        .map(|(r, crowns)| match crowns.len() {
            x if x == max_difficulty as usize => r,
            // i.e. restriction applies to all crowns
            1 => format!("{}-Crown: {}", crowns[0], r),
            _ => panic!("Restrictions don't apply to all crowns!"),
            // kinda misleading, specifically it's doesn't apply to all or 1,
            // but that's way too long-winded for something that might not
            // happen
        })
        .filter(|s| s != FOUR_CROWN_DEFAULT)
        .collect()
}

/// Get a list of stage restrictions if they exist.
fn get_restriction_list(stage: &Stage) -> Option<Vec<String>> {
    let restrictions = stage.restrictions.as_ref()?;
    if restrictions.is_empty() || restrictions == &[FOUR_CROWN_DEFAULT_RESTRICTION] {
        return None;
    }

    if restrictions.len() == 1 {
        let restriction = &restrictions[0];
        if restriction.crowns_applied != Crowns::All
            && stage.crown_data.is_some()
            && (stage.crown_data.as_ref().unwrap().max_difficulty > non_zero_u8(1)
                || restriction.crowns_applied != Crowns::One(non_zero_u8(1)))
        // TODO use if-let chains in 2024 version when it stabilises
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
    Some(get_multi_restriction(restrictions, max_difficulty))
}

/// Get restrictions for Stage Info template (including no continues).
pub fn restrictions_info(stage: &Stage) -> Option<TemplateParameter> {
    const PARAM_NAME: &str = "restriction";

    let restrictions = get_restriction_list(stage);
    let Some(r) = restrictions else {
        return stage
            .is_no_continues
            .then(|| TemplateParameter::new(PARAM_NAME, "[[No Continues]]".to_string()));
    };

    let mut buf = r.join("<br>\n");
    if stage.is_no_continues {
        buf.write_str("<br>\n[[No Continues]]").infallible_write();
    }

    Some(TemplateParameter::new(PARAM_NAME, buf))
}

// TODO fixed_formation.csv
/// Get content of restrictions section.
pub fn restrictions_section(stage: &Stage) -> String {
    let restrictions = match get_restriction_list(stage) {
        None => return String::new(),
        Some(r) => r,
    };

    if restrictions.len() == 1 {
        return restrictions.into_iter().next().unwrap();
    }

    let mut buf = String::new();
    for restriction in restrictions {
        buf.write_str("*").infallible_write();
        buf.write_str(&restriction).infallible_write();
        buf.write_str("\n").infallible_write();
    }
    buf.truncate(buf.len() - "\n".len());
    buf
}

/// Get content of rules section.
pub fn rules(stage: &Stage) -> String {
    let Some(rules) = &stage.rules else {
        return String::new();
    };
    if let Some(name) = &rules.rule_name_label {
        let mut buf = "{{ColosseumRule|".to_string();
        buf += name.as_str();
        buf += "}}";
        return buf;
    }

    match rules.contents_type {
        ContentsType::Anni12 => {
            assert_eq!(&rules.rule_type, &[RuleType::TrustFund([4500])]);
            String::from("{{StageRule|12thAnni}}")
        }
        ContentsType::Colosseum => unreachable!(
            "Should have reached {:?} in earlier code.",
            rules.contents_type
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::stage::parsed::stage::RestrictionCrowns,
        meta::stage::{stage_id::StageID, variant::StageVariantID as T},
    };

    #[test]
    fn no_restrictions() {
        let boxing_clever =
            Stage::from_id_current(StageID::from_components(T::Event, 50, 1)).unwrap();
        assert_eq!(boxing_clever.restrictions, None);
        assert_eq!(restrictions_info(&boxing_clever), None);
        assert_eq!(&restrictions_section(&boxing_clever), "");
    }

    #[test]
    fn no_continues() {
        let realm_of_carnage =
            Stage::from_id_current(StageID::from_components(T::Event, 117, 0)).unwrap();
        assert_eq!(realm_of_carnage.restrictions, None);
        assert_eq!(
            restrictions_info(&realm_of_carnage),
            Some(TemplateParameter::new("restriction", "[[No Continues]]"))
        );
        assert_eq!(&restrictions_section(&realm_of_carnage), "");
    }

    #[test]
    fn only_4_crown_restrictions() {
        let earthshaker = Stage::from_id_current(StageID::from_components(T::SoL, 0, 0)).unwrap();
        assert_eq!(
            earthshaker.restrictions.as_ref().unwrap(),
            &[FOUR_CROWN_DEFAULT_RESTRICTION]
        );
        assert_eq!(restrictions_info(&earthshaker), None);
        assert_eq!(&restrictions_section(&earthshaker), "");
    }

    #[test]
    fn restriction_rarity_1() {
        let sighter_star =
            Stage::from_id_current(StageID::from_components(T::MainChapters, 8, 24)).unwrap();
        assert_eq!(
            restrictions_info(&sighter_star),
            Some(TemplateParameter::new(
                "restriction",
                "Rarity: Only [[:Category:Special Cats|Special]], [[:Category:Rare Cats|Rare]] and [[:Category:Super Rare Cats|Super Rare]]"
            ))
        );
        assert_eq!(
            &restrictions_section(&sighter_star),
            "Rarity: Only [[:Category:Special Cats|Special]], [[:Category:Rare Cats|Rare]] and [[:Category:Super Rare Cats|Super Rare]]"
        );
    }

    #[test]
    fn restriction_rarity_2() {
        let babies_first =
            Stage::from_id_current(StageID::from_components(T::Event, 375, 0)).unwrap();
        assert_eq!(
            restrictions_info(&babies_first),
            Some(TemplateParameter::new(
                "restriction",
                "Rarity: Only [[:Category:Normal Cats|Normal]] and [[:Category:Uber Rare Cats|Uber Rare]]<br>\n[[No Continues]]"
            ))
        );
        assert_eq!(
            &restrictions_section(&babies_first),
            "Rarity: Only [[:Category:Normal Cats|Normal]] and [[:Category:Uber Rare Cats|Uber Rare]]"
        );
    }

    #[test]
    fn restriction_rarity_3() {
        let somolon =
            Stage::from_id_current(StageID::from_components(T::MainChapters, 8, 37)).unwrap();
        assert_eq!(
            restrictions_info(&somolon),
            Some(TemplateParameter::new(
                "restriction",
                "Rarity: Only [[:Category:Special Cats|Special]]"
            ))
        );
        assert_eq!(
            &restrictions_section(&somolon),
            "Rarity: Only [[:Category:Special Cats|Special]]"
        );
    }

    #[test]
    fn restriction_rarity_4() {
        let wahwah = Stage::from_id_current(StageID::from_components(T::Event, 158, 0)).unwrap();
        assert_eq!(
            restrictions_info(&wahwah),
            Some(TemplateParameter::new(
                "restriction",
                "Rarity: Only [[:Category:Normal Cats|Normal]], [[:Category:Uber Rare Cats|Uber Rare]] and [[:Category:Legend Rare Cats|Legend Rare]]<br>\n[[No Continues]]"
            ))
        );
        assert_eq!(
            &restrictions_section(&wahwah),
            "Rarity: Only [[:Category:Normal Cats|Normal]], [[:Category:Uber Rare Cats|Uber Rare]] and [[:Category:Legend Rare Cats|Legend Rare]]"
        );
    }

    #[test]
    fn restriction_deploy_limit() {
        let wrath_w_cyclone =
            Stage::from_id_current(StageID::from_components(T::Event, 176, 0)).unwrap();
        assert_eq!(
            restrictions_info(&wrath_w_cyclone),
            Some(TemplateParameter::new(
                "restriction",
                "Max # of Deployable Cats: 10"
            ))
        );
        assert_eq!(
            &restrictions_section(&wrath_w_cyclone),
            "Max # of Deployable Cats: 10"
        );
    }

    #[test]
    fn restriction_rows() {
        let uranus =
            Stage::from_id_current(StageID::from_components(T::MainChapters, 7, 7)).unwrap();
        assert_eq!(
            restrictions_info(&uranus),
            Some(TemplateParameter::new(
                "restriction",
                "Deploy from Row 1 only"
            ))
        );
        assert_eq!(&restrictions_section(&uranus), "Deploy from Row 1 only");
    }

    #[test]
    fn restriction_min_cost_1() {
        let saturn =
            Stage::from_id_current(StageID::from_components(T::MainChapters, 7, 3)).unwrap();
        assert_eq!(
            restrictions_info(&saturn),
            Some(TemplateParameter::new(
                "restriction",
                "Cat Deploy Cost: Only 300¢ or more"
            ))
        );
        assert_eq!(
            &restrictions_section(&saturn),
            "Cat Deploy Cost: Only 300¢ or more"
        );
    }

    #[test]
    fn restriction_min_cost_2() {
        let skelling =
            Stage::from_id_current(StageID::from_components(T::MainChapters, 7, 40)).unwrap();
        assert_eq!(
            restrictions_info(&skelling),
            Some(TemplateParameter::new(
                "restriction",
                "Cat Deploy Cost: Only 1,200¢ or more"
            ))
        );
        assert_eq!(
            &restrictions_section(&skelling),
            "Cat Deploy Cost: Only 1,200¢ or more"
        );
    }

    #[test]
    fn restriction_max_cost_1() {
        let buutara =
            Stage::from_id_current(StageID::from_components(T::MainChapters, 6, 27)).unwrap();
        assert_eq!(
            restrictions_info(&buutara),
            Some(TemplateParameter::new(
                "restriction",
                "Cat Deploy Cost: Only 1,200¢ or less"
            ))
        );
        assert_eq!(
            &restrictions_section(&buutara),
            "Cat Deploy Cost: Only 1,200¢ or less"
        );
    }

    #[test]
    fn restriction_max_cost_2() {
        let catseye_nebula =
            Stage::from_id_current(StageID::from_components(T::MainChapters, 6, 13)).unwrap();
        assert_eq!(
            restrictions_info(&catseye_nebula),
            Some(TemplateParameter::new(
                "restriction",
                "Cat Deploy Cost: Only 4,000¢ or less"
            ))
        );
        assert_eq!(
            &restrictions_section(&catseye_nebula),
            "Cat Deploy Cost: Only 4,000¢ or less"
        );
    }

    #[test]
    fn restriction_only_cat() {
        let finale = Stage::from_id_current(StageID::from_components(T::Collab, 209, 0)).unwrap();
        assert_eq!(
            restrictions_info(&finale),
            Some(TemplateParameter::new(
                "restriction",
                "Unit Restriction: Only [[Cat (Normal Cat)|Cat]]"
            ))
        );
        assert_eq!(
            &restrictions_section(&finale),
            "Unit Restriction: Only [[Cat (Normal Cat)|Cat]]"
        );
    }

    #[test]
    fn restriction_only_jra() {
        let final_race =
            Stage::from_id_current(StageID::from_components(T::Collab, 179, 0)).unwrap();
        assert_eq!(
            restrictions_info(&final_race),
            Some(TemplateParameter::new(
                "restriction",
                "Unit Restriction: Only [[Cat Giraffe Modoki (Special Cat)|Cat Giraffe Modoki]], [[Catnip Tricky (Special Cat)|Catnip Tricky]] and [[Catnip Dragon (Special Cat)|Catnip Dragon]]"
            ))
        );
        assert_eq!(
            &restrictions_section(&final_race),
            "Unit Restriction: Only [[Cat Giraffe Modoki (Special Cat)|Cat Giraffe Modoki]], [[Catnip Tricky (Special Cat)|Catnip Tricky]] and [[Catnip Dragon (Special Cat)|Catnip Dragon]]"
        );
    }

    #[test]
    fn restriction_exclude_madoka() {
        let sorry = Stage::from_id_current(StageID::from_components(T::Collab, 178, 4)).unwrap();
        assert_eq!(
            restrictions_info(&sorry),
            Some(TemplateParameter::new(
                "restriction",
                "Unit Restriction: Cannot use [[Homura Akemi (Uber Rare Cat)|Homura Akemi]] and [[Li'l Homura (Special Cat)|Li'l Homura]]<br>\n[[No Continues]]"
            ))
        );
        assert_eq!(
            &restrictions_section(&sorry),
            "Unit Restriction: Cannot use [[Homura Akemi (Uber Rare Cat)|Homura Akemi]] and [[Li'l Homura (Special Cat)|Li'l Homura]]"
        );
    }

    #[test]
    fn restriction_multiple_cotc() {
        let black_hole =
            Stage::from_id_current(StageID::from_components(T::MainChapters, 7, 46)).unwrap();
        assert_eq!(
            restrictions_info(&black_hole),
            Some(TemplateParameter::new(
                "restriction",
                "Rarity: Only [[:Category:Special Cats|Special]], [[:Category:Rare Cats|Rare]], \
                [[:Category:Uber Rare Cats|Uber Rare]] and \
                [[:Category:Legend Rare Cats|Legend Rare]]<br>\n\
                Max # of Deployable Cats: 10"
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
        let feathered = Stage::from_id_current(StageID::from_components(T::Collab, 86, 0)).unwrap();

        assert_eq!(
            restrictions_info(&feathered),
            Some(TemplateParameter::new(
                "restriction",
                "Rarity: Only [[:Category:Normal Cats|Normal]], [[:Category:Special Cats|Special]] and [[:Category:Rare Cats|Rare]]<br>\n\
                4-Crown: Max # of Deployable Cats: 10"
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
        let revenge_r_cyclone =
            Stage::from_id_current(StageID::from_components(T::Event, 169, 1)).unwrap();
        assert_eq!(revenge_r_cyclone.restrictions, Some(vec![]));
        assert_eq!(restrictions_info(&revenge_r_cyclone), None);
        assert_eq!(restrictions_section(&revenge_r_cyclone), "");
    }

    #[test]
    fn multiple_same_restriction() {
        let vanguard_veteran =
            Stage::from_id_current(StageID::from_components(T::Event, 213, 0)).unwrap();
        let restrictions = vanguard_veteran.restrictions.as_ref().unwrap();
        assert_eq!(restrictions[0], restrictions[1]);
        assert_eq!(
            restrictions_info(&vanguard_veteran),
            Some(TemplateParameter::new(
                "restriction",
                "Max # of Deployable Cats: 1"
            ))
        );
        assert_eq!(
            restrictions_section(&vanguard_veteran),
            "Max # of Deployable Cats: 1"
        );
    }

    fn dessert_witch_restriction(crowns: RestrictionCrowns) -> Restriction {
        Restriction {
            stages_applied: RestrictionStages::All,
            crowns_applied: crowns,
            rarity: None,
            deploy_limit: None,
            rows: None,
            min_cost: None,
            max_cost: None,
            charagroup: Some(CharaGroup {
                group_type: CharaGroupType::CannotUse,
                units: [440].into(),
            }),
        }
    }
    #[test]
    fn multiple_restrictions_with_4_crown() {
        let afraid_nothing =
            Stage::from_id_current(StageID::from_components(T::Collab, 221, 3)).unwrap();
        assert_eq!(
            afraid_nothing.restrictions.as_ref().unwrap(),
            &[
                dessert_witch_restriction(RestrictionCrowns::One(NonZero::new(1).unwrap())),
                dessert_witch_restriction(RestrictionCrowns::One(NonZero::new(2).unwrap())),
                dessert_witch_restriction(RestrictionCrowns::One(NonZero::new(3).unwrap())),
                {
                    let mut restriction =
                        dessert_witch_restriction(RestrictionCrowns::One(NonZero::new(4).unwrap()));
                    restriction.rarity = FOUR_CROWN_DEFAULT_RESTRICTION.rarity;
                    restriction
                },
            ]
        );
        assert_eq!(
            restrictions_info(&afraid_nothing),
            Some(TemplateParameter::new(
                "restriction",
                "Unit Restriction: Cannot use [[Bebe (Uber Rare Cat)|Bebe]]"
            ))
        );
        assert_eq!(
            &restrictions_section(&afraid_nothing),
            "Unit Restriction: Cannot use [[Bebe (Uber Rare Cat)|Bebe]]"
        );
    }

    #[test]
    #[should_panic = "Crown 1 is duplicated in vec [1, 1, 1, 1]."]
    fn test_assert_all_restrictions_unique() {
        let restrictions: &[(std::string::String, Vec<u8>)] = &[
            ("Rarity: Short lines".into(), [1, 1, 1, 1].into()),
            ("Max # of Deployable Cats: 10".into(), [4].into()),
        ];
        assert_all_restrictions_unique(restrictions);
    }

    #[test]
    fn rule_trust_fund() {
        let trust_fund_2 =
            Stage::from_id_current(StageID::from_components(T::Colosseum, 0, 1)).unwrap();
        assert_eq!(rules(&trust_fund_2), "{{ColosseumRule|Trust Fund}}");
    }

    #[test]
    fn rule_12th_anniversary() {
        let doge_disturbance_last =
            Stage::from_id_current(StageID::from_components(T::Extra, 71, 9)).unwrap();
        assert_eq!(rules(&doge_disturbance_last), "{{StageRule|12thAnni}}");
    }
}
