//! Defines a template parameter class.

use std::io::Write;

/// Representation of a wikitext template parameter.
pub struct TemplateParameter {
    key: &'static [u8],
    value: Vec<u8>,
}
// maybe key and value could be generic args but that's for later.
impl TemplateParameter {
    /// Create a parameter.
    pub fn new(key: &'static [u8], value: Vec<u8>) -> TemplateParameter {
        Self { key, value }
    }
    /// Convert object to a vec of u8s.
    pub fn to_u8s(&self) -> Vec<u8>{
        let mut buf = vec![];
        for value in [b"|", self.key, b" = ", &self.value]{
            buf.write(value).unwrap();
        }
        buf
    }
}
