use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TabberType {
    Tabber,
    SubTabber,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TabberTab {
    pub title: String,
    pub content: String,
}
impl TabberTab {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}
impl Display for TabberTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\n{}", self.title, self.content)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tabber {
    pub ttype: TabberType,
    pub content: Vec<TabberTab>,
}
impl Tabber {
    pub fn new(ttype: TabberType, content: Vec<TabberTab>) -> Self {
        Self { ttype, content }
    }
}
impl Display for Tabber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (open, mid, close) = match self.ttype {
            TabberType::Tabber => ("<tabber>", "|-|", "</tabber>"),
            TabberType::SubTabber => ("{{#tag:tabber", "{{!}}-{{!}}", "}}"),
        };

        write!(f, "{open}\n")?;

        let mut iter = self.content.iter().peekable();
        while let Some(tab) = iter.next() {
            write!(f, "{tab}\n")?;
            if iter.peek().is_some() {
                write!(f, "\n{mid}\n")?;
            }
        }

        write!(f, "{close}")
    }
}

pub enum SectionTitle {
    Blank,
    H2(String),
}

pub struct Section {
    pub title: SectionTitle,
    pub content: String,
}
impl Section {
    pub fn blank(content: String) -> Self {
        Self {
            title: SectionTitle::Blank,
            content,
        }
    }

    pub fn h2(title: String, content: String) -> Self {
        Self {
            title: SectionTitle::H2(title),
            content,
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
