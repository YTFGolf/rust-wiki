//! Tabber part of page.

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
/// Type of tabber.
pub enum TabberType {
    /// `<tabber>`.
    #[default]
    Tabber,
    /// `{{#tag:tabber|`.
    SubTabber,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Tab of a tabber.
pub struct TabberTab {
    /// Tab's title.
    pub title: String,
    /// Tab's content.
    pub content: String,
}
impl TabberTab {
    /// Create tabber tab.
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}
impl Display for TabberTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\n{}", self.title, self.content)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
/// Tabber container.
pub struct Tabber {
    /// Type of tabber.
    pub ttype: TabberType,
    /// Tabs inside tabber.
    pub content: Vec<TabberTab>,
}
impl Tabber {
    /// Create new tabber.
    pub fn new(ttype: TabberType, content: Vec<TabberTab>) -> Self {
        Self { ttype, content }
    }
}
impl Display for Tabber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.content.len() == 1 {
            return write!(f, "{}", self.content[0].content);
        }

        let (open, mid, close) = match self.ttype {
            TabberType::Tabber => ("<tabber>", "|-|", "</tabber>"),
            TabberType::SubTabber => ("{{#tag:tabber", "{{!}}-{{!}}", "}}"),
        };

        writeln!(f, "{open}")?;

        let mut iter = self.content.iter().peekable();
        while let Some(tab) = iter.next() {
            writeln!(f, "{tab}")?;
            if iter.peek().is_some() {
                write!(f, "\n{mid}\n")?;
            }
        }

        write!(f, "{close}")
    }
}
