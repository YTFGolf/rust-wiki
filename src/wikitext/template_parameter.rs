//! Defines a template parameter class.

use std::fmt::Write;

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
for key, value in re.findall(r'\|([\w ]+) = ((?:.|\n)*?)(?=\n\||$)', lines):
    value = value.replace('\n', "\\n\\" + f"\n{' ' * indent * 4}")
    if type == "opt":
        print(f'Some(TemplateParameter::new("{key}", "{value}".to_string()))')
    else:
        print(f'TemplateParameter::new("{key}", "{value}".to_string()),')

*/
