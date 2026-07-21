//! Utility functions for templates.
use crate::{
    game_data::cat::parsed::unitbuy::AncientEggInfo,
    wiki_data::cat_data::{CAT_DATA, CatName},
};
use std::borrow::Cow;
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
    pub fn name(self, id: u32) -> Cow<'static, str> {
        self.name_option(id).unwrap()
    }

    /// Name of unit in this form, `None` if form does not have a name.
    pub fn name_option(self, id: u32) -> Option<Cow<'static, str>> {
        let id = id as usize;
        let cat = match CAT_DATA.try_get_cat(id) {
            Some(c) => c,
            None => return Some(Cow::Owned(CatName::get_placeholder(id, self as usize + 1))),
        };
        match self {
            Self::Normal => Some(Cow::Borrowed(&cat.normal)),
            Self::Evolved => cat.evolved.as_ref().map(|n| Cow::Owned(n.clone())),
            Self::True => cat.true_form.as_ref().map(|n| Cow::Owned(n.clone())),
            Self::Ultra => cat.ultra.as_ref().map(|n| Cow::Owned(n.clone())),
            // borrow checker pls be nice
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

    /// Cat's wiki appearance image.
    pub fn wiki_appearance(self, id: u32, eggs: &AncientEggInfo) -> String {
        match self {
            CatForm::Normal => match eggs {
                AncientEggInfo::None => format!("{id:03} 1.png"),
                AncientEggInfo::Egg { normal, .. } => format!("m {normal:03}.png"),
            },
            CatForm::Evolved => match eggs {
                AncientEggInfo::None => format!("{id:03} 2.png"),
                AncientEggInfo::Egg { evolved, .. } => format!("m {evolved:03}.png"),
            },
            CatForm::True => format!("{id:03} 3.png"),
            CatForm::Ultra => format!("{id:03} 4.png"),
        }
    }
}
