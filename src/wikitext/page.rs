//! Represents the whole wikitext page.

use crate::wikitext::section::Section;
use std::fmt::Display;

/// Represents a wikitext page.
pub struct Page(Vec<Section>);

impl Page {
    /// Create new [`Page`] with sections.
    fn new(sections: Vec<Section>) -> Self {
        Self(sections)
    }

    /// Create [`Page`] with no sections.
    pub fn blank() -> Self {
        Self::new(vec![])
    }
}

impl Page {
    /// Push a new section to the page.
    pub fn push(&mut self, section: Section) {
        self.0.push(section);
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        let Some(first) = iter.next() else {
            return Ok(());
        };

        write!(f, "{first}")?;
        for rest in iter {
            write!(f, "\n\n{rest}")?;
        }

        Ok(())
    }
}
