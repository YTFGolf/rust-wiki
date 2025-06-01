use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::scripts::cat_info::stats::form::get_first_form,
    wiki_data::cat_data::CAT_DATA,
    wikitext::template::{Template, TemplateParameter},
};

pub fn get_template(cat: Cat) {
    let stats = &cat.forms.stats[0];
    let anims = &cat.forms.anims[0];

    let name = &CAT_DATA.get_cat(cat.id).normal;
    let form = get_first_form(&cat, stats, anims);

    let mut t = Template::named("Cat Stats");
    type P = TemplateParameter;

    t.push_params(P::new("Normal Form name", name));
    t.push_params(P::new("Hp Normal", form.base_hp));
    t.push_params(P::new("Atk Power Normal", form.base_atk));

    t.push_params(P::new("Atk Range Normal", form.range));
    t.push_params(P::new("Attack Frequency Normal", form.attack_cycle));
    t.push_params(P::new("Movement Speed Normal", form.speed));
    t.push_params(P::new("Knockback Normal", form.knockback));
    t.push_params(P::new("Attack Animation Normal", form.animation));
    t.push_params(P::new("Recharging Time Normal", form.recharge));
    t.push_params(P::new("Hp Normal Lv.MAX", form.hp_max));
    t.push_params(P::new("Atk Power Normal Lv.MAX", form.atk_max));
    t.push_params(P::new("Attack type Normal", form.attack_type));
    t.push_params(P::new("Special Ability Normal", form.abilities));

    println!("{t}");
}
