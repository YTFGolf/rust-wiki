use std::iter::zip;

use super::form::Form;
use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::scripts::cat_info::stats::form::get_form,
    wiki_data::cat_data::CAT_DATA,
    wikitext::template::{Template, TemplateParameter},
};

fn write_stats(t: &mut Template, form_name: &str, form: Form) {
    type P = TemplateParameter;
    let f = form_name;

    t.push_params(P::new(format!("Atk Range {f}"), form.range));
    t.push_params(P::new(format!("Attack Frequency {f}"), form.attack_cycle));
    t.push_params(P::new(format!("Movement Speed {f}"), form.speed));
    t.push_params(P::new(format!("Knockback {f}"), form.knockback));
    t.push_params(P::new(format!("Attack Animation {f}"), form.animation));
    t.push_params(P::new(format!("Recharging Time {f}"), form.recharge));
    t.push_params(P::new(format!("Hp {f} Lv.MAX"), form.hp_max));
    t.push_params(P::new(format!("Atk Power {f} Lv.MAX"), form.atk_max));
    t.push_params(P::new(format!("Attack type {f}"), form.attack_type));
    t.push_params(P::new(format!("Special Ability {f}"), form.abilities));
}

fn add_all_forms(t: &mut Template, cat: &Cat) {
    type P = TemplateParameter;

    let mut iter = zip(&cat.forms.stats, &cat.forms.anims).take(cat.forms.amt_forms);

    let Some((stats, anims)) = iter.next() else {
        return;
    };
    let name = &CAT_DATA.get_cat(cat.id).normal;
    t.push_params(P::new("Normal Form name", name));

    let form = get_form(&cat, stats, anims);
    t.push_params(P::new("Hp Normal", form.base_hp));
    t.push_params(P::new("Atk Power Normal", form.base_atk));
    write_stats(t, "Normal", form.other);

    let Some((stats, anims)) = iter.next() else {
        return;
    };

    let name = &CAT_DATA.get_cat(cat.id).evolved.as_ref().unwrap();
    t.push_params(P::new("Evolved Form name", *name));
    let form = get_form(&cat, stats, anims);
    write_stats(t, "Evolved", form.other);

    let Some((stats, anims)) = iter.next() else {
        return;
    };

    let name = &CAT_DATA.get_cat(cat.id).true_form.as_ref().unwrap();
    t.push_params(P::new("True Form name", *name));
    let form = get_form(&cat, stats, anims);
    write_stats(t, "True", form.other);

    let Some((stats, anims)) = iter.next() else {
        return;
    };

    let name = &CAT_DATA.get_cat(cat.id).ultra.as_ref().unwrap();
    t.push_params(P::new("Ultra Form name", *name));
    let form = get_form(&cat, stats, anims);
    write_stats(t, "Ultra", form.other);
}

pub fn get_template(cat: Cat) {
    let mut t = Template::named("Cat Stats");

    add_all_forms(&mut t, &cat);

    println!("{t}");
}
