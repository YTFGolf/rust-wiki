//! Get manual Cat Stats template.

use super::super::form::Form;
use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::scripts::cat_info::{
        form_util::CatForm,
        stats::form::{get_form, write_level_and_plus},
    },
    wiki_data::cat_data::CatName,
    wikitext::template::{Template, TemplateParameter}, Config,
};
use std::iter::zip;

fn write_stats(t: &mut Template, form_name: &str, form: Form) {
    type P = TemplateParameter;
    let f = form_name;

    let is_normal_form = matches!(form_name, "Normal");

    if !is_normal_form {
        t.push_params(P::new(format!("{f} Health"), form.hp_max.clone()));
        t.push_params(P::new(format!("{f} Attack Power"), form.atk_max.clone()));
    }

    t.push_params(P::new(format!("{f} Attack Range"), form.range));
    t.push_params(P::new(format!("{f} Attack Frequency"), form.attack_cycle));
    t.push_params(P::new(format!("{f} Movement Speed"), form.speed));
    t.push_params(P::new(format!("{f} Knockback"), form.knockback));
    t.push_params(P::new(format!("{f} Attack Animation"), form.animation));
    t.push_params(P::new(format!("{f} Recharge Time"), form.recharge));

    if is_normal_form {
        t.push_params(P::new(format!("{f} MaxHP"), form.hp_max));
        t.push_params(P::new(format!("{f} MaxAttack"), form.atk_max));
    }

    t.push_params(P::new(format!("{f} Attack Type"), form.attack_type));
    t.push_params(P::new(format!("{f} Abilities"), form.abilities));
}

fn add_all_forms(t: &mut Template, cat: &Cat) {
    type P = TemplateParameter;
    type F = CatForm;

    let forms = [F::Normal, F::Evolved, F::True, F::Ultra];
    let iter = cat.forms.iter();

    for (form_variant, stats_and_anims) in zip(forms, iter) {
        let name = form_variant.name(cat.id);
        let name = CatName::clean_cat_name(name);

        let form_name = form_variant.as_str();
        let (stats, anims) = stats_and_anims;
        t.push_params(P::new(format!("{form_name} Name"), name));

        let form = get_form(cat, stats, anims, form_variant);

        t.push_params(form.stats_level.map(|lv| {
            let name = format!("{form_name} Stats Level");
            TemplateParameter::new(name, lv)
        }));

        if form_variant == F::Normal {
            t.push_params(P::new("Normal Health", form.base_hp));
            t.push_params(P::new("Normal Attack Power", form.base_atk));
        }
        write_stats(t, form_name, form.other);
    }
}

/// Get manual cat stats template.
pub fn stats_manual(_config: &Config, cat: &Cat) -> Template {
    let mut t = Template::named("Cat Stats Manual");

    add_all_forms(&mut t, cat);
    let max_level = {
        let max = &cat.unitbuy.max_levels;
        let mut buf = String::from("Lv.");
        write_level_and_plus(&mut buf, max.max_nat, max.max_plus);
        buf
    };
    t.push_params(TemplateParameter::new("Lv.MAX", max_level));

    t
}
