//! Utility functions for templates.
use crate::wiki_data::cat_data::CAT_DATA;
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
