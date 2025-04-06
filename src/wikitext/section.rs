use std::{borrow::Cow, fmt::Display};

type StringValue = Cow<'static, str>;

pub enum SectionTitle {
    Blank,
    H2(StringValue),
}

pub struct Section {
    pub title: SectionTitle,
    pub content: StringValue,
}
impl Section {
    pub fn blank<T>(content: T) -> Self
    where
        T: Into<StringValue>,
    {
        Self {
            title: SectionTitle::Blank,
            content: content.into(),
        }
    }

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
impl Section {
    pub fn content(&self) -> &str {
        &self.content
    }
}
impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.title {
            SectionTitle::Blank => (),
            SectionTitle::H2(title) => writeln!(f, "=={title}==")?,
        };
        f.write_str(&self.content)
    }
}
