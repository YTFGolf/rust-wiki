use crate::{
    data::stage::parsed::stage::{Restriction, RestrictionCrowns, Stage},
    wikitext::template_parameter::TemplateParameter,
};
use std::num::NonZeroU8;

const NON_ZERO_FOUR: NonZeroU8 = match NonZeroU8::new(4) {
    Some(v) => v,
    None => [][0],
};
const FOUR_CROWN_DEFAULT_RESTRICTION: Restriction = Restriction {
    crowns_applied: RestrictionCrowns::One(NON_ZERO_FOUR),
    rarity: NonZeroU8::new(0b000110),
    deploy_limit: None,
    rows: None,
    min_cost: None,
    max_cost: None,
    charagroup: None,
};

fn get_restriction_list(restriction: &Restriction) -> Vec<Vec<u8>> {
    todo!()
}

pub fn restrictions_info(stage: &Stage) -> Option<TemplateParameter> {
    let restrictions = stage.restrictions.as_ref()?;

    if restrictions.len() == 1 {
        if restrictions == &[FOUR_CROWN_DEFAULT_RESTRICTION] {
            return None;
        }
    }

    todo!()
}

pub fn restrictions_section(_stage: &Stage) -> Vec<u8> {
    vec![]
}

// Realm of Carnage
// earthshaker
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
    }

    #[test]
    fn only_4_crown_restrictions() {
        let earthshaker = Stage::new("sol 0 0").unwrap();
        assert_eq!(
            earthshaker.restrictions.as_ref().unwrap(),
            &[FOUR_CROWN_DEFAULT_RESTRICTION]
        );
        assert_eq!(restrictions_info(&earthshaker), None);
    }
}
