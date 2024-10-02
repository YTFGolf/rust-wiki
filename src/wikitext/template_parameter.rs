//! Defines a template parameter class.

use std::{fmt::Write, io::Write as uncoolWrite};

#[derive(Debug, PartialEq)]
/// Representation of a wikitext template parameter. Legacy implementation.
pub struct TemplateParameterU8 {
    key: &'static [u8],
    value: Vec<u8>,
}
// maybe key and value could be generic args but that's for later.
impl TemplateParameterU8 {
    /// Create a parameter.
    pub fn new(key: &'static [u8], value: Vec<u8>) -> TemplateParameterU8 {
        Self { key, value }
    }
    /// Convert object to a vec of u8s.
    pub fn to_u8s(&self) -> Vec<u8> {
        let mut buf = vec![];
        for value in [b"|", self.key, b" = ", &self.value] {
            buf.write(value).unwrap();
        }
        buf
    }
}

impl From<TemplateParameterU8> for String {
    fn from(value: TemplateParameterU8) -> Self {
        String::from_utf8(value.to_u8s()).unwrap()
    }
}

#[derive(Debug, PartialEq)]
/// Representation of a wikitext template parameter.
pub struct TemplateParameter {
    key: &'static str,
    value: String,
}
impl TemplateParameter {
    /// Create a parameter.
    pub fn new(key: &'static str, value: String) -> TemplateParameter {
        Self { key, value }
    }
    /// Get string representation of object.
    pub fn to_string(&self) -> String {
        let mut buf: String = "".into();
        for value in ["|", self.key, " = ", &self.value] {
            buf.write_str(value).unwrap();
        }
        buf
    }
}

impl From<TemplateParameter> for String {
    fn from(value: TemplateParameter) -> Self {
        value.to_string()
    }
}

/*
# Code to turn any stage's infobox parameters into a TemplateParameter (mainly
# for testing).
import re
def get_lines():
    lines = input('Input things: ')
    new = 1
    while new:
        new = input()
        lines = f'{lines}\n{new}'
    return lines[:-1]

type = "opt"
if type == "opt":
    indent = 4
else:
    indent = 5
lines = get_lines()
for key, value in re.findall(r'\|(\w+) = ((?:.|\n)*?)(?=\n\||$)', lines):
    value = value.replace('\n', "\\n\\" + f"\n{' ' * indent * 4}")
    if type == "opt":
        print(f'Some(TemplateParameter::new(b"{key}", b"{value}".to_vec()))')
    else:
        print(f'TemplateParameter::new(b"{key}", b"{value}".to_vec()),')

*/
