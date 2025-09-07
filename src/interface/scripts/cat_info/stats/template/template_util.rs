//! Utility functions for templates.

use crate::wiki_data::cat_data::CAT_DATA;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Cat's form.
pub enum CatForm {
    /// Normal form.
    Normal,
    /// Evolved form.
    Evolved,
    /// True form.
    True,
    /// Ultra form.
    Ultra,
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
        match self {
            Self::Normal => &CAT_DATA.get_cat(id).normal,
            Self::Evolved => &CAT_DATA.get_cat(id).evolved.as_ref().unwrap(),
            Self::True => &CAT_DATA.get_cat(id).true_form.as_ref().unwrap(),
            Self::Ultra => &CAT_DATA.get_cat(id).ultra.as_ref().unwrap(),
        }
    }
}
