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
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn no_combos() {
        let kerihime = Cat::from_wiki_id(26, &TEST_CONFIG.version).unwrap();
        let sect = combos_section(&kerihime, &TEST_CONFIG);
        assert_eq!(sect, None);
    }

    #[test]
    fn one_combo() {
        let zeus = Cat::from_wiki_id(257, &TEST_CONFIG.version).unwrap();
        let sect = combos_section(&zeus, &TEST_CONFIG).unwrap().to_string();

        assert_eq!(
            sect,
            "{{Combos\n\
            |{{CatCombo|Punish the Lewd|\"Resistant\" Effect UP (Sm)|Gentleman Cat|Thunder God Zeus|jpname=変態紳士に天罰を}}\n\
            }}"
        );
    }

    #[test]
    fn in_removed_combos() {
        let cat = Cat::from_wiki_id(0, &TEST_CONFIG.version).unwrap();
        let sect = combos_section(&cat, &TEST_CONFIG).unwrap().to_string();

        assert_eq!(
            sect,
            "{{Combos\n\
            |{{CatCombo|Cat Army|Worker Cat Start Level UP (Sm)|Cat|Tank Cat|Axe Cat|Gross Cat|Cow Cat|jpname=にゃんこ軍団}}\n\
            |{{CatCombo|Mo' Hawks|\"Knockback\" Effect UP (Sm)|Mohawk Cat|Crazed Dragon Cat|jpname=モヒカン愛好家}}\n\
            |{{CatCombo|Smiles at Cats|\"Freeze\" Effect UP (Sm)|Cat|Coppermine, type κ|jpname=ネコの前で見せる笑顔}}\n\
            |{{CatCombo|Rich and Poor|Starting Money UP (Sm)|Cat|Gold Cat|jpname=庶民と金持ち}}\n\
            |{{CatCombo|Black & White|Worker Cat Max UP (Sm)|Killer Cat|Cat|jpname=ブラック＆ホワイト}}\n\
            }}"
        );
    }

    #[test]
    fn jp_only_combos() {
        let dn105 = Cat::from_wiki_id(542, &TEST_CONFIG.version).unwrap();
        // combo doesn't have EN name so needs to fall back to JP name
        let sect = combos_section(&dn105, &TEST_CONFIG).unwrap().to_string();

        assert_eq!(
            sect,
            "{{Combos\n\
            |{{CatCombo|先生になりたいでにゃ～る|Cat Cannon Recharge UP (Sm)|Cat Boy|jpname=先生になりたいでにゃ～る}}\n\
            }}"
        );
    }
}
