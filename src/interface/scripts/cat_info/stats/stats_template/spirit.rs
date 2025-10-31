//! Get manual Cat Stats template.

use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::scripts::cat_info::{form_util::CatForm, stats::form::get_form},
    wikitext::template::{Template, TemplateParameter},
};

/// Get spirit stats template.
pub fn stats_spirit(cat: &Cat) -> Template {
    type P = TemplateParameter;
    const KAMIKAZE: &str = "{{AbilityIcon|Kamikaze}} [[Special Abilities#Kamikaze|Kamikaze]] (Attacks once, then disappears from the battlefield)<br>\n";

    let mut t = Template::named("SpiritStats");
    assert_eq!(cat.forms.amt_forms, 1);
    let (stats, anims) = (&cat.forms.stats[0], &cat.forms.anims[0]);

    let id = cat.id;
    t.push_params(P::new("Spirit CRO", format!("{id:03}")));
    t.push_params(P::new("Spirit Image", format!("{id:03} 1.png")));

    let form = get_form(cat, stats, anims, CatForm::Normal);
    t.push_params(P::new("Spirit Atk", form.other.atk_max));
    t.push_params(P::new("Spirit Range", form.other.range));
    t.push_params(P::new("Spirit Animation", form.other.animation));
    t.push_params(P::new("Spirit Target", form.other.attack_type));

    let abilities = form.other.abilities;
    assert!(abilities.starts_with(KAMIKAZE));
    t.push_params(P::new(
        "Spirit Ability",
        abilities[KAMIKAZE.len()..].to_owned(),
    ));

    t
}
