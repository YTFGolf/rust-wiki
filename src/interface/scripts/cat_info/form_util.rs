//! Utility functions for templates.
use crate::{game_data::cat::parsed::unitbuy::AncientEggInfo, wiki_data::cat_data::CAT_DATA};
use strum::FromRepr;

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromRepr)]
/// Cat's form.
pub enum CatForm {
    /// Normal form.
    Normal = 0,
    /// Evolved form.
    Evolved = 1,
    /// True form.
    True = 2,
    /// Ultra form.
    Ultra = 3,
}
impl CatForm {
    /// String representation of form name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Evolved => "Evolved",
            Self::True => "True",
            Self::Ultra => "Ultra",
        }
    }

    /// Name of given unit in this form.
    pub fn name(self, id: u32) -> &'static str {
        self.name_option(id).unwrap()
    }

    /// Name of unit in this form, `None` if form does not have a name.
    pub fn name_option(self, id: u32) -> Option<&'static String> {
        match self {
            Self::Normal => Some(&CAT_DATA.get_cat(id).normal),
            Self::Evolved => CAT_DATA.get_cat(id).evolved.as_ref(),
            Self::True => CAT_DATA.get_cat(id).true_form.as_ref(),
            Self::Ultra => CAT_DATA.get_cat(id).ultra.as_ref(),
        }
    }
}

impl CatForm {
    /// [`Self::deploy_icon`] with no `.png` extension.
    pub fn deploy_icon_no_ext(self, id: u32, eggs: &AncientEggInfo) -> String {
        match self {
            CatForm::Normal => match eggs {
                AncientEggInfo::None => format!("Uni{id:03} f00"),
                AncientEggInfo::Egg { normal, .. } => format!("Uni{normal:03} m00"),
            },
            CatForm::Evolved => match eggs {
                AncientEggInfo::None => format!("Uni{id:03} c00"),
                AncientEggInfo::Egg { evolved, .. } => format!("Uni{evolved:03} m01"),
            },
            CatForm::True => format!("Uni{id:03} s00"),
            CatForm::Ultra => format!("Uni{id:03} u00"),
        }
    }
    /// Cat's in-battle deploy icon.
    pub fn deploy_icon(self, id: u32, eggs: &AncientEggInfo) -> String {
        self.deploy_icon_no_ext(id, eggs) + ".png"
    }
}
