//! Defines a template parameter class.

use std::{borrow::Cow, fmt::Display};

#[derive(Debug, PartialEq)]
/// Representation of a wikitext template parameter.
pub struct TemplateParameter {
    key: Cow<'static, str>,
    value: String,
}
impl TemplateParameter {
    /// Create a parameter.
    pub fn new<T: Into<Cow<'static, str>>>(key: T, value: String) -> Self {
        Self {
            key: key.into(),
            value,
        }
    }
}
impl Display for TemplateParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(
            f,
            "|{key} = {value}",
            key = self.key,
            value = self.value
        )?)
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
