//! Deals with sections of a page.
use std::{borrow::Cow, fmt::Display};

type StringValue = Cow<'static, str>;

/// Title to use for the section.
pub enum SectionTitle {
    /// No title (useful for top of page for example).
    Blank,
    /// H2 heading.
    H2(StringValue),
}

/// Page section.
pub struct Section {
    /// Section's title.
    pub title: SectionTitle,
    /// Section's content.
    pub content: StringValue,
}
impl Section {
    /// Create section with no title.
    pub fn blank<T>(content: T) -> Self
    where
        T: Into<StringValue>,
    {
        Self {
            title: SectionTitle::Blank,
            content: content.into(),
        }
    }

    /// Create section with h2 heading.
    pub fn h2<T, U>(title: T, content: U) -> Self
    where
        T: Into<StringValue>,
        U: Into<StringValue>,
    {
        Self {
            title: SectionTitle::H2(title.into()),
            content: content.into(),
        }
    }
}
impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.title {
            SectionTitle::Blank => (),
            SectionTitle::H2(title) => writeln!(f, "=={title}==")?,
        }
        f.write_str(&self.content)
    }
}
