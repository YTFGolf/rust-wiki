//! Module for Cat Stats template.

use super::super::form::Form;
use crate::{
    Config,
    game_data::cat::parsed::{anim::CatFormAnimData, cat::Cat, stats::form::CatFormStats},
    interface::{
        error_handler::InfallibleWrite,
        scripts::cat_info::{
            form_util::CatForm,
            stats::form::{get_form, write_level_and_plus},
        },
    },
    wiki_data::cat_data::CatName,
    wikitext::template::{Template, TemplateParameter},
};
use std::{fmt::Write, iter::zip};

fn write_val_stats(t: &mut Template, form_name: &str, form: Form) {
    type P = TemplateParameter;
    let f = form_name;
    // this template is so inconsistent

    let is_normal_form = matches!(form_name, "Normal");

    if !is_normal_form {
        t.push_params(P::new(format!("val-{f}-Health"), form.hp_max.clone()));
        t.push_params(P::new(
            format!("val-{f}-Attack Power"),
            form.atk_max.clone(),
        ));
    }

    t.push_params(P::new(format!("val-{f}-Attack Range"), form.range));
    t.push_params(P::new(
        format!("val-{f}-Attack Frequency"),
        form.attack_cycle,
    ));
    t.push_params(P::new(format!("val-{f}-Movement Speed"), form.speed));
    t.push_params(P::new(format!("val-{f}-Knockback"), form.knockback));
    t.push_params(P::new(format!("val-{f}-Attack Animation"), form.animation));
    t.push_params(P::new(format!("val-{f}-Recharge Time"), form.recharge));

    if is_normal_form {
        t.push_params(P::new(format!("val-{f}-MaxHP"), form.hp_max));
        t.push_params(P::new(format!("val-{f}-MaxAttack"), form.atk_max));
    }

    t.push_params(P::new(format!("{f} Attack Type"), form.attack_type));
    t.push_params(P::new(format!("{f} Abilities"), form.abilities));
}

fn write_stats(t: &mut Template, form_name: &str, stats: &CatFormStats, anims: &CatFormAnimData) {
    type P = TemplateParameter;
    let f = form_name;

    t.push_params(P::new(format!("{f} Base HP"), stats.hp.to_string()));
    t.push_params(P::new(
        format!("{f} Base AP"),
        stats
            .attack
            .hits
            .iter()
            .map(|hit| hit.damage.to_string())
            .collect::<Vec<_>>()
            .join(","),
    ));

    t.push_params(P::new(
        format!("{f} Range"),
        stats.attack.standing_range.to_string(),
    ));
    t.push_params(P::new(
        format!("{f} Foreswing"),
        stats
            .attack
            .hits
            .iter()
            .map(|hit| hit.foreswing.to_string())
            .collect::<Vec<_>>()
            .join(","),
    ));
    t.push_params(P::new(
        format!("{f} Attack Cooldown"),
        stats.attack.cooldown.to_string(),
    ));
    t.push_params(P::new(
        format!("{f} Attack Length"),
        anims.attack.length().to_string(),
    ));
    t.push_params(P::new(format!("{f} Speed"), stats.speed.to_string()));
    t.push_params(P::new(format!("{f} KB"), stats.kb.to_string()));
    t.push_params(P::new(format!("{f} Cost"), stats.price.to_string()));
    t.push_params(P::new(
        format!("{f} Recharge"),
        (stats.respawn_half * 2).to_string(),
    ));
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

        write_stats(t, form_name, stats, anims);
        if form_variant == F::Normal {
            t.push_params(P::new("val-Normal-Health", form.base_hp));
            t.push_params(P::new("val-Normal-Attack Power", form.base_atk));
        }
        write_val_stats(t, form_name, form.other);
    }
}

fn get_scaling(cat: &Cat) -> String {
    let mut buf = String::new();

    let mut iter = cat.unitlevel.iter().peekable();
    let mut last_count = 1;
    let mut last_id = iter.next().expect("length is always 20");
    while let Some(scale) = iter.next() {
        if last_id == scale {
            last_count += 1;
            continue;
        }

        match last_count {
            ..=0 => unreachable!(),
            1 => write!(buf, "{last_id}").infallible_write(),
            2.. => write!(buf, "{last_id}x{last_count}").infallible_write(),
        }

        if iter.peek().is_some() {
            buf += ",";
        }

        last_count = 1;
        last_id = scale;
    }

    match last_count {
        ..=0 => (),
        1 => write!(buf, "{last_id}").infallible_write(),
        2.. => write!(buf, "{last_id}x{last_count}").infallible_write(),
    }

    buf
}

/// Get full template.
pub fn stats_0o2(cat: &Cat, config: &Config) -> Template {
    let mut t = Template::named("Cat Stats 0.2");

    add_all_forms(&mut t, cat);
    let max_level = {
        let max = &cat.unitbuy.max_levels;
        let mut buf = String::from("Lv.");
        write_level_and_plus(&mut buf, max.max_nat, max.max_plus);
        buf
    };

    let scaling = get_scaling(cat);

    let slots = config
        .version
        .current_version()
        .get_cached_file::<EquipmentSlotContainer>();
    let cat_has_slots = match slots.get_slot_item(cat.id.try_into().unwrap()) {
        None => false,
        Some(slot) => slot.amt_slots > 0,
    };

    t.push_params(TemplateParameter::new("Lv.MAX", max_level));
    t.push_params(TemplateParameter::new("Scaling", scaling));
    t.push_params(TemplateParameter::new("Orb", cat_has_slots.to_string()));
    if !config.cat_info.stats_hide_validation {
        t.push_params(TemplateParameter::new("validation", "on"));
    }

    let (title, mut items) = t.deconstruct();
    if config.cat_info.stats_hide_validation {
        items.retain(|x| !x.key.starts_with("val-"));
    } else {
        items.sort_by_key(|x| x.key.starts_with("val-"));
    }

    Template::new(title, items)
}
