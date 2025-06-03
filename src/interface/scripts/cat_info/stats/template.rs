use super::form::Form;
use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::scripts::cat_info::stats::form::{get_form, write_level_and_plus},
    wiki_data::cat_data::{CatName, CAT_DATA},
    wikitext::template::{Template, TemplateParameter},
};
use std::iter::zip;

fn write_stats(t: &mut Template, form_name: &str, form: Form) {
    type P = TemplateParameter;
    let f = form_name;
    // this template is so inconsistent

    let is_normal_form = match form_name {
        "Normal" => true,
        _ => false,
    };

    if !is_normal_form {
        t.push_params(P::new(format!("Hp {f}"), form.hp_max.clone()));
        t.push_params(P::new(format!("Atk Power {f}"), form.atk_max.clone()));
    }

    t.push_params(P::new(format!("Atk Range {f}"), form.range));
    t.push_params(P::new(format!("Attack Frequency {f}"), form.attack_cycle));
    t.push_params(P::new(format!("Movement Speed {f}"), form.speed));
    t.push_params(P::new(format!("Knockback {f}"), form.knockback));
    t.push_params(P::new(format!("Attack Animation {f}"), form.animation));
    t.push_params(P::new(format!("Recharging Time {f}"), form.recharge));

    if is_normal_form {
        t.push_params(P::new(format!("Hp {f} Lv.MAX"), form.hp_max));
        t.push_params(P::new(format!("Atk Power {f} Lv.MAX"), form.atk_max));
    }

    t.push_params(P::new(format!("Attack type {f}"), form.attack_type));
    t.push_params(P::new(format!("Special Ability {f}"), form.abilities));
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
    t.push_params(P::new("Normal Form name", name));

    let form = get_form(cat, stats, anims, 1);
    t.push_params(get_stats_level("1st stats Level", form.stats_level));
    t.push_params(P::new("Hp Normal", form.base_hp));
    t.push_params(P::new("Atk Power Normal", form.base_atk));
    write_stats(t, "Normal", form.other);

    let Some((stats, anims)) = iter.next() else {
        return;
    };

    let name = &CAT_DATA.get_cat(cat.id).evolved.as_ref().unwrap();
    let name = CatName::clean_cat_name(name);
    t.push_params(P::new("Evolved Form name", name));

    let form = get_form(cat, stats, anims, 2);
    t.push_params(get_stats_level("2nd stats Level", form.stats_level));
    write_stats(t, "Evolved", form.other);

    let Some((stats, anims)) = iter.next() else {
        return;
    };

    let name = &CAT_DATA.get_cat(cat.id).true_form.as_ref().unwrap();
    let name = CatName::clean_cat_name(name);
    t.push_params(P::new("True Form name", name));

    let form = get_form(cat, stats, anims, 3);
    t.push_params(get_stats_level("3rd stats Level", form.stats_level));
    write_stats(t, "True", form.other);

    let Some((stats, anims)) = iter.next() else {
        return;
    };

    // t.push_params("4th stats Level");
    let name = &CAT_DATA.get_cat(cat.id).ultra.as_ref().unwrap();
    let name = CatName::clean_cat_name(name);
    t.push_params(P::new("Ultra Form name", name));
    let form = get_form(cat, stats, anims, 4);
    write_stats(t, "Ultra", form.other);
}

pub fn get_template(cat: Cat) {
    let mut t = Template::named("Cat Stats");

    add_all_forms(&mut t, &cat);
    let max_level = {
        let max = cat.unitbuy.max_levels;
        let mut buf = String::from("Lv.");
        write_level_and_plus(&mut buf, max.max_nat, max.max_plus);
        buf
    };
    t.push_params(TemplateParameter::new("Lv.MAX", max_level));

    println!("{t}");
}
