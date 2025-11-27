//! Deals with sections of a page.
use std::{borrow::Cow, fmt::Display};

type StringValue = Cow<'static, str>;

/// Title to use for the section.
#[derive(Debug, PartialEq, Eq)]
pub enum SectionTitle {
    /// No title (useful for top of page for example).
    Blank,
    /// H2 heading.
    H2(StringValue),
    /// H3 heading.
    H3(StringValue),
    /// H4 heading.
    H4(StringValue),
    /// H5 heading.
    H5(StringValue),
    /// H6 heading.
    H6(StringValue),
}

/// Page section.
#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    /// Section's title.
    pub title: SectionTitle,
    /// Section's content.
    pub content: StringValue,
}
impl Section {
    /// Create section.
    pub fn new(title: SectionTitle, content: StringValue) -> Self {
        Self { title, content }
    }

    /// Create section with no title.
    pub fn blank<T>(content: T) -> Self
    where
        T: Into<StringValue>,
    {
        Self::new(SectionTitle::Blank, content.into())
    }

    /// Create section with h2 heading.
    pub fn h2<T, U>(title: T, content: U) -> Self
    where
        T: Into<StringValue>,
        U: Into<StringValue>,
    {
        Self::new(SectionTitle::H2(title.into()), content.into())
    }

    /// Create section with h3 heading.
    pub fn h3<T, U>(title: T, content: U) -> Self
    where
        T: Into<StringValue>,
        U: Into<StringValue>,
    {
        Self::new(SectionTitle::H3(title.into()), content.into())
    }

    /// Create section with h4 heading.
    pub fn h4<T, U>(title: T, content: U) -> Self
    where
        T: Into<StringValue>,
        U: Into<StringValue>,
    {
        Self::new(SectionTitle::H4(title.into()), content.into())
    }

    /// Create section with h5 heading.
    pub fn h5<T, U>(title: T, content: U) -> Self
    where
        T: Into<StringValue>,
        U: Into<StringValue>,
    {
        Self::new(SectionTitle::H5(title.into()), content.into())
    }

    /// Create section with h6 heading.
    pub fn h6<T, U>(title: T, content: U) -> Self
    where
        T: Into<StringValue>,
        U: Into<StringValue>,
    {
        Self::new(SectionTitle::H6(title.into()), content.into())
    }
}
impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.title {
            SectionTitle::Blank => (),
            SectionTitle::H2(title) => writeln!(f, "=={title}==")?,
            SectionTitle::H3(title) => writeln!(f, "==={title}===")?,
            SectionTitle::H4(title) => writeln!(f, "===={title}====")?,
            SectionTitle::H5(title) => writeln!(f, "====={title}=====")?,
            SectionTitle::H6(title) => writeln!(f, "======{title}======")?,
        }
        f.write_str(self.content.trim())
    }
}
