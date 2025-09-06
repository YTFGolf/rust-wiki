use super::form::Form;
use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::scripts::cat_info::stats::form::{get_form, write_level_and_plus},
    wiki_data::cat_data::{CAT_DATA, CatName},
    wikitext::template::{Template, TemplateParameter},
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

fn get_stats_level(name: &'static str, value: Option<String>) -> Option<TemplateParameter> {
    Some(TemplateParameter::new(name, value?))
}

fn add_all_forms(t: &mut Template, cat: &Cat) {
    type P = TemplateParameter;

    let mut iter = zip(&cat.forms.stats, &cat.forms.anims).take(cat.forms.amt_forms);

    let Some((stats, anims)) = iter.next() else {
        return;
    };

    let name = &CAT_DATA.get_cat(cat.id).normal;
    let name = CatName::clean_cat_name(name);
    t.push_params(P::new("Normal Name", name));

    let form = get_form(cat, stats, anims, 1);
    t.push_params(get_stats_level("Normal Stats Level", form.stats_level));
    t.push_params(P::new("Normal Health", form.base_hp));
    t.push_params(P::new("Normal Attack Power", form.base_atk));
    write_stats(t, "Normal", form.other);

    let Some((stats, anims)) = iter.next() else {
        return;
    };

    let name = &CAT_DATA.get_cat(cat.id).evolved.as_ref().unwrap();
    let name = CatName::clean_cat_name(name);
    t.push_params(P::new("Evolved Name", name));

    let form = get_form(cat, stats, anims, 2);
    t.push_params(get_stats_level("Evolved Stats Level", form.stats_level));
    write_stats(t, "Evolved", form.other);

    let Some((stats, anims)) = iter.next() else {
        return;
    };

    let name = &CAT_DATA.get_cat(cat.id).true_form.as_ref().unwrap();
    let name = CatName::clean_cat_name(name);
    t.push_params(P::new("True Name", name));

    let form = get_form(cat, stats, anims, 3);
    t.push_params(get_stats_level("True Stats Level", form.stats_level));
    write_stats(t, "True", form.other);

    let Some((stats, anims)) = iter.next() else {
        return;
    };

    // t.push_params("Ultra Stats Level");
    let name = &CAT_DATA.get_cat(cat.id).ultra.as_ref().unwrap();
    let name = CatName::clean_cat_name(name);
    t.push_params(P::new("Ultra Name", name));
    let form = get_form(cat, stats, anims, 4);
    write_stats(t, "Ultra", form.other);
}

pub fn get_old_template(cat: &Cat) -> Template {
    let mut t = Template::named("Cat Stats Manual");

    add_all_forms(&mut t, &cat);
    let max_level = {
        let max = &cat.unitbuy.max_levels;
        let mut buf = String::from("Lv.");
        write_level_and_plus(&mut buf, max.max_nat, max.max_plus);
        buf
    };
    t.push_params(TemplateParameter::new("Lv.MAX", max_level));

    t
}
