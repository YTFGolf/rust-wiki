use num_format::{Locale, WriteFormatted};
use crate::{
    data::stage::parsed::stage::{Restriction, RestrictionCrowns as Crowns, Stage},
    wikitext::template_parameter::TemplateParameter,
};
use std::{
    io::Write,
    num::{NonZero, NonZeroU8},
};

const fn non_zero_u8(value: u8) -> NonZero<u8> {
    match NonZeroU8::new(value) {
        Some(v) => v,
        None => panic!("Value must be non-zero!"),
    }
}
const FOUR_CROWN_DEFAULT_RESTRICTION: Restriction = Restriction {
    crowns_applied: Crowns::One(non_zero_u8(4)),
    rarity: NonZeroU8::new(0b000110),
    deploy_limit: None,
    rows: None,
    min_cost: None,
    max_cost: None,
    charagroup: None,
};

fn get_rarity_restriction(rarity: NonZero<u8>) -> Vec<u8> {
    let rarities: Vec<&[u8]> = (0..6)
        .filter_map(|i| {
            let rarity_bit = u8::from(rarity) & (1 << i);
            if rarity_bit == 0 {
                return None;
            };

            let rarity: &[u8] = match i {
                0 => b"[[:Category:Normal Cats|Normal]]",
                1 => b"[[:Category:Special Cats|Special]]",
                2 => b"[[:Category:Rare Cats|Rare]]",
                3 => b"[[:Category:Super Rare Cats|Super Rare]]",
                4 => b"[[:Category:Uber Rare Cats|Uber Rare]]",
                5 => b"[[:Category:Legend Rare Cats|Legend Rare]]",
                _ => unreachable!(),
            };
            Some(rarity)
        })
        .collect();

    let mut buf = b"Rarity: Only ".to_vec();
    if rarities.len() == 1 {
        buf.write(rarities[0]).unwrap();
    } else {
        let (last, first) = rarities.split_last().unwrap();
        let grouped = first.join(b", ".as_slice());
        buf.write(&grouped).unwrap();
        buf.write(b" and ").unwrap();
        buf.write(last).unwrap();
    }
    buf
}

fn get_single_restriction(restriction: &Restriction) -> Vec<Vec<u8>> {
    let mut restrictions = vec![];

    if let Some(rarity) = restriction.rarity {
        let buf = get_rarity_restriction(rarity);
        restrictions.push(buf);
    }
    if let Some(limit) = restriction.deploy_limit {
        let mut buf = vec![];
        write!(buf, "Max # of Deployable Cats: {}", limit).unwrap();
        restrictions.push(buf);
    }
    if let Some(row) = restriction.rows {
        let mut buf = vec![];
        write!(buf, "Deploy from Row {} only", row).unwrap();
        restrictions.push(buf);
    }
    if let Some(min) = restriction.min_cost {
        let mut buf = vec![];
        buf.write(b"Cat Deploy Cost: Only ").unwrap();
        buf.write_formatted(&min, &Locale::en).unwrap();
        buf.write(b"\xA2 or more").unwrap();
        // \xA2 = ¢
        restrictions.push(buf);
    }
    if let Some(max) = restriction.max_cost {
        let mut buf = vec![];
        buf.write(b"Cat Deploy Cost: Only ").unwrap();
        buf.write_formatted(&max, &Locale::en).unwrap();
        buf.write(b"\xA2 or less").unwrap();
        // \xA2 = ¢
        restrictions.push(buf);
    }
    if let Some(group) = restriction.charagroup {
        todo!()
    }

    restrictions
}

fn get_restriction_list(stage: &Stage) -> Option<Vec<Vec<u8>>> {
    let restrictions = stage.restrictions.as_ref()?;

    if restrictions.len() == 1 {
        if restrictions == &[FOUR_CROWN_DEFAULT_RESTRICTION] {
            return None;
        }

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

    assert_eq!(
        restrictions.len(),
        2,
        "Restrictions length is more than 2! (stage: {stage:?})"
    );
    // There may be cases where restrictions len > 2, but this relies on PONOS
    // being bad at their jobs so I'll deal with it when it comes up.
    todo!()
}

pub fn restrictions_info(stage: &Stage) -> Option<TemplateParameter> {
    const PARAM_NAME: &[u8] = b"restriction";

    let restrictions = get_restriction_list(stage);
    let r = match restrictions {
        None => {
            if !stage.is_no_continues {
                return None;
            }
            return Some(TemplateParameter::new(
                &PARAM_NAME,
                b"[[No Continues]]".to_vec(),
            ));
        }
        Some(r) => r,
    };

    let mut buf = r.join(b"<br>\n".as_slice());
    if stage.is_no_continues {
        buf.write(b"<br>\n[[No Continues]]").unwrap();
    }

    Some(TemplateParameter::new(PARAM_NAME, buf))
}

pub fn restrictions_section(stage: &Stage) -> Vec<u8> {
    let restrictions = match get_restriction_list(stage) {
        None => return vec![],
        Some(r) => r,
    };

    if restrictions.len() == 1 {
        return restrictions.into_iter().next().unwrap();
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_restrictions() {
        let boxing_clever = Stage::new("s 50 1").unwrap();
        assert_eq!(boxing_clever.restrictions, None);
        assert_eq!(restrictions_info(&boxing_clever), None);
        assert_eq!(restrictions_section(&boxing_clever), vec![]);
    }

    #[test]
    fn no_continues() {
        let realm_of_carnage = Stage::new("s 117 0").unwrap();
        assert_eq!(realm_of_carnage.restrictions, None);
        assert_eq!(
            restrictions_info(&realm_of_carnage),
            Some(TemplateParameter::new(
                b"restriction",
                b"[[No Continues]]".to_vec()
            ))
        );
        assert_eq!(restrictions_section(&realm_of_carnage), vec![]);
    }

    #[test]
    fn only_4_crown_restrictions() {
        let earthshaker = Stage::new("sol 0 0").unwrap();
        assert_eq!(
            earthshaker.restrictions.as_ref().unwrap(),
            &[FOUR_CROWN_DEFAULT_RESTRICTION]
        );
        assert_eq!(restrictions_info(&earthshaker), None);
        assert_eq!(restrictions_section(&earthshaker), vec![]);
    }

    #[test]
    fn restriction_rarity_1() {
        let sighter_star = Stage::new("cotc 3 24").unwrap();
        assert_eq!( restrictions_info (&sighter_star), Some(TemplateParameter::new( b"restriction", b"Rarity: Only [[:Category:Special Cats|Special]], [[:Category:Rare Cats|Rare]] and [[:Category:Super Rare Cats|Super Rare]]".to_vec() )) );
        assert_eq!( &restrictions_section(&sighter_star), b"Rarity: Only [[:Category:Special Cats|Special]], [[:Category:Rare Cats|Rare]] and [[:Category:Super Rare Cats|Super Rare]]" );
    }

    #[test]
    fn restriction_rarity_2() {
        let babies_first = Stage::new("s 375 0").unwrap();
        assert_eq!( restrictions_info (&babies_first), Some(TemplateParameter::new( b"restriction", b"Rarity: Only [[:Category:Normal Cats|Normal]] and [[:Category:Uber Rare Cats|Uber Rare]]<br>\n[[No Continues]]".to_vec() )) );
        assert_eq!( &restrictions_section(&babies_first), b"Rarity: Only [[:Category:Normal Cats|Normal]] and [[:Category:Uber Rare Cats|Uber Rare]]" );
    }

    #[test]
    fn restriction_rarity_3() {
        let somolon = Stage::new("cotc 3 37").unwrap();
        println!("{}", String::from(restrictions_info(&somolon).unwrap()));
        assert_eq!(
            restrictions_info(&somolon),
            Some(TemplateParameter::new(
                b"restriction",
                b"Rarity: Only [[:Category:Special Cats|Special]]".to_vec()
            ))
        );
        assert_eq!(
            &restrictions_section(&somolon),
            b"Rarity: Only [[:Category:Special Cats|Special]]"
        );
    }

    #[test]
    fn restriction_rarity_4() {
        let wahwah = Stage::new("s 158 0").unwrap();
        assert_eq!( restrictions_info (&wahwah), Some(TemplateParameter::new( b"restriction", b"Rarity: Only [[:Category:Normal Cats|Normal]], [[:Category:Uber Rare Cats|Uber Rare]] and [[:Category:Legend Rare Cats|Legend Rare]]<br>\n[[No Continues]]".to_vec() )) );
        assert_eq!( &restrictions_section(&wahwah), b"Rarity: Only [[:Category:Normal Cats|Normal]], [[:Category:Uber Rare Cats|Uber Rare]] and [[:Category:Legend Rare Cats|Legend Rare]]" );
    }

    #[test]
    fn restriction_deploy_limit() {
        let wrath_w_cyclone = Stage::new("s 176 0").unwrap();
        assert_eq!(
            restrictions_info(&wrath_w_cyclone),
            Some(TemplateParameter::new(
                b"restriction",
                b"Max # of Deployable Cats: 10".to_vec()
            ))
        );
        assert_eq!(
            &restrictions_section(&wrath_w_cyclone),
            b"Max # of Deployable Cats: 10"
        );
    }

    #[test]
    fn restriction_rows() {
        let uranus = Stage::new("cotc 2 7").unwrap();
        assert_eq!(
            restrictions_info(&uranus),
            Some(TemplateParameter::new(
                b"restriction",
                b"Deploy from Row 1 only".to_vec()
            ))
        );
        assert_eq!(&restrictions_section(&uranus), b"Deploy from Row 1 only");
    }

    #[test]
    fn restriction_min_cost_1() {
        let saturn = Stage::new("cotc 2 3").unwrap();
        assert_eq!(
            restrictions_info(&saturn),
            Some(TemplateParameter::new(
                b"restriction",
                b"Cat Deploy Cost: Only 300\xA2 or more".to_vec()
            ))
        );
        assert_eq!(
            &restrictions_section(&saturn),
            b"Cat Deploy Cost: Only 300\xA2 or more"
        );
    }

    #[test]
    fn restriction_min_cost_2() {
        let skelling = Stage::new("cotc 2 40").unwrap();
        assert_eq!(
            restrictions_info(&skelling),
            Some(TemplateParameter::new(
                b"restriction",
                b"Cat Deploy Cost: Only 1,200\xA2 or more".to_vec()
            ))
        );
        assert_eq!(
            &restrictions_section(&skelling),
            b"Cat Deploy Cost: Only 1,200\xA2 or more"
        );
    }

    #[test]
    fn restriction_max_cost_1() {
        let buutara = Stage::new("cotc 1 27").unwrap();
        assert_eq!(
            restrictions_info(&buutara),
            Some(TemplateParameter::new(
                b"restriction",
                b"Cat Deploy Cost: Only 1,200\xA2 or less".to_vec()
            ))
        );
        assert_eq!(
            &restrictions_section(&buutara),
            b"Cat Deploy Cost: Only 1,200\xA2 or less"
        );
    }

    #[test]
    fn restriction_max_cost_2() {
        let catseye_nebula = Stage::new("cotc 1 13").unwrap();
        assert_eq!(
            restrictions_info(&catseye_nebula),
            Some(TemplateParameter::new(
                b"restriction",
                b"Cat Deploy Cost: Only 4,000\xA2 or less".to_vec()
            ))
        );
        assert_eq!(
            &restrictions_section(&catseye_nebula),
            b"Cat Deploy Cost: Only 4,000\xA2 or less"
        );
    }

    #[test]
    fn restriction_only_cat() {
        let finale = Stage::new("c 209 0").unwrap();
        assert_eq!(
            restrictions_info(&finale),
            Some(TemplateParameter::new(
                b"restriction",
                b"Unit Restriction: Only [[Cat (Normal Cat)|Cat]]".to_vec()
            ))
        );
        assert_eq!(
            &restrictions_section(&finale),
            b"Unit Restriction: Only [[Cat (Normal Cat)|Cat]]"
        );
    }

    // cotc stages esp. black hole
}
