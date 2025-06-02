use super::form::Form;
use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::scripts::cat_info::stats::form::get_form,
    wiki_data::cat_data::CAT_DATA,
    wikitext::template::{Template, TemplateParameter},
};

pub fn write_stats(t: &mut Template, form_name: &str, form: Form) {
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

pub fn get_template(cat: Cat) {
    let stats = &cat.forms.stats[0];
    let anims = &cat.forms.anims[0];

    let mut t = Template::named("Cat Stats");
    type P = TemplateParameter;

    let name = &CAT_DATA.get_cat(cat.id).normal;
    t.push_params(P::new("Normal Form name", name));

    let form = get_form(&cat, stats, anims);
    t.push_params(P::new("Hp Normal", form.base_hp));
    t.push_params(P::new("Atk Power Normal", form.base_atk));
    write_stats(&mut t, "Normal", form.other);

    println!("{t}");
}
