//! Combos template.

use crate::{
    game_data::cat::{
        parsed::cat::Cat,
        raw::{
            combo::{ComboData, CombosDataContainer},
            combo_local::{ComboEffects, ComboIntensities, ComboNames},
        },
    },
    interface::{
        config::Config, error_handler::InfallibleWrite, scripts::cat_info::form_util::CatForm,
    },
    wikitext::section::Section,
};
use std::fmt::Write;

fn fmt_combo(i: usize, combo: &ComboData, config: &Config) -> String {
    let mut buf = String::from("{{CatCombo|");

    let en_names = config.version.en().get_cached_file::<ComboNames>();
    let jp_names = config.version.jp().get_cached_file::<ComboNames>();
    let combo_name = match en_names.combo_name(i) {
        None | Some("") => jp_names.combo_name(i).unwrap(),
        Some(name) => name,
    };
    buf += combo_name;

    let en_effects = config.version.en().get_cached_file::<ComboEffects>();
    let combo_effect = match en_effects.effect_name(combo.effect_num.into()) {
        None | Some("") => {
            let jp_effects = config.version.jp().get_cached_file::<ComboEffects>();
            jp_effects.effect_name(combo.effect_num.into()).unwrap()
        }
        Some(effect) => effect,
    };
    buf += "|";
    buf += combo_effect;

    let en_intensities = config.version.en().get_cached_file::<ComboIntensities>();
    let combo_intensity = match en_intensities.intensity_name(combo.intensity_num.into()) {
        None | Some("") => {
            let jp_intensities = config.version.jp().get_cached_file::<ComboIntensities>();
            jp_intensities
                .intensity_name(combo.intensity_num.into())
                .unwrap()
        }
        Some(intensity) => intensity,
    };
    buf += combo_intensity;

    for unit in combo.units.iter() {
        let form = CatForm::from_repr(unit.form.try_into().unwrap()).unwrap();
        buf += "|";
        buf += form.name(unit.id.try_into().unwrap());
    }

    buf += "|jpname=";
    buf += jp_names.combo_name(i).unwrap();
    buf += "}}";

    buf
}

/// Section for combos if the cat has any.
pub fn combos_section(cat: &Cat, config: &Config) -> Option<Section> {
    let combo_container = config
        .version
        .current_version()
        .get_cached_file::<CombosDataContainer>();

    let mut has_cat = combo_container.by_cat_id(cat.id.try_into().unwrap());

    let first = has_cat.next()?;
    let mut buf = String::from("{{Combos\n");

    writeln!(buf, "|{fmt}", fmt = fmt_combo(first.0, first.1, config)).infallible_write();
    for rest in has_cat {
        writeln!(buf, "|{fmt}", fmt = fmt_combo(rest.0, rest.1, config)).infallible_write();
    }

    Some(Section::blank(buf + "}}"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_combos() {
        todo!()
    }

    #[test]
    fn one_combo() {
        todo!()
    }

    #[test]
    fn in_removed_combos() {
        // cat, 0
        todo!()
    }

    #[test]
    fn multiple_combos() {
        // ururun, 130
        todo!()
    }

    #[test]
    fn jp_only_combos() {
        // dn105, 542
        todo!()
    }
}
