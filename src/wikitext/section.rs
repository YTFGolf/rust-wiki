use std::fmt::Display;

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
