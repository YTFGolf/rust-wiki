use super::form::Form;
use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::scripts::cat_info::stats::form::{get_form, write_level_and_plus},
    wiki_data::cat_data::{CAT_DATA, CatName},
    wikitext::{
        template::{Template, TemplateParameter},
        text_utils::get_ordinal,
    },
};
use std::iter::zip;

fn write_stats(t: &mut Template, form_name: &str, form: Form) {
    type P = TemplateParameter;
    let f = form_name;
    // this template is so inconsistent

    let is_normal_form = matches!(form_name, "Normal");

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

fn add_all_forms(t: &mut Template, cat: &Cat) {
    type P = TemplateParameter;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    enum CatForm {
        Normal = 1,
        Evolved = 2,
        True = 3,
        Ultra = 4,
    }
    impl CatForm {
        fn as_str(self) -> &'static str {
            match self {
                Self::Normal => "Normal",
                Self::Evolved => "Evolved",
                Self::True => "True",
                Self::Ultra => "Ultra",
            }
        }

        fn name(self, id: u32) -> &'static str {
            match self {
                Self::Normal => &CAT_DATA.get_cat(id).normal,
                Self::Evolved => &CAT_DATA.get_cat(id).evolved.as_ref().unwrap(),
                Self::True => &CAT_DATA.get_cat(id).true_form.as_ref().unwrap(),
                Self::Ultra => &CAT_DATA.get_cat(id).ultra.as_ref().unwrap(),
            }
        }
    }
    use CatForm as F;

    let forms = [F::Normal, F::Evolved, F::True, F::Ultra];
    let iter = zip(&cat.forms.stats, &cat.forms.anims).take(cat.forms.amt_forms);

    for (form_variant, stats_and_anims) in zip(forms, iter) {
        let name = form_variant.name(cat.id);
        let name = CatName::clean_cat_name(name);

        let form_str = form_variant.as_str();
        let (stats, anims) = stats_and_anims;
        t.push_params(P::new(format!("{form_str} Form name"), name));

        let form = get_form(cat, stats, anims, form_variant as u8);

        let ord = get_ordinal(form_variant as u32);
        t.push_params(form.stats_level.map(|lv| {
            let name = format!("{ord} stats Level");
            TemplateParameter::new(name, lv)
        }));

        if form_variant == F::Normal {
            t.push_params(P::new("Hp Normal", form.base_hp));
            t.push_params(P::new("Atk Power Normal", form.base_atk));
        }
        write_stats(t, form_str, form.other);
    }
}

pub fn get_template(cat: &Cat) -> Template {
    let mut t = Template::named("Cat Stats");

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
