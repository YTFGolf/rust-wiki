use crate::{
    data::stage::parsed::stage::{Restriction, RestrictionCrowns as Crowns, Stage},
    wikitext::template_parameter::TemplateParameter,
};
use std::num::{NonZero, NonZeroU8};

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

fn get_restriction_list(restriction: &Restriction) -> Vec<Vec<u8>> {
    let _ = restriction;
    todo!()
}

pub fn restrictions_info(stage: &Stage) -> Option<TemplateParameter> {
    const PARAM_NAME: &[u8] = b"restriction";
    let restrictions = stage.restrictions.as_ref()?;

    if restrictions.len() == 1 {
        if restrictions == &[FOUR_CROWN_DEFAULT_RESTRICTION] {
            return None;
        }

        let restriction = &restrictions[0];
        if restriction.crowns_applied != Crowns::All
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

        return Some(TemplateParameter::new(
            PARAM_NAME,
            get_restriction_list(restriction).join(b"<br>\n".as_slice()),
        ));
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

pub fn restrictions_section(_stage: &Stage) -> Vec<u8> {
    vec![]
}

// finale
// cotc stages esp. black hole

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_restrictions() {
        let realm_of_carnage = Stage::new("s 117 0").unwrap();
        assert_eq!(realm_of_carnage.restrictions, None);
        assert_eq!(restrictions_info(&realm_of_carnage), None);
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
    fn restriction_only_cat() {
        let finale = Stage::new("c 209 0").unwrap();
        assert_eq!(
            restrictions_info(&finale),
            Some(TemplateParameter::new(
                b"restriction",
                b"Unit Restriction: Only [[Cat (Normal Cat)|Cat]]".to_vec()
            ))
        )
    }
}
