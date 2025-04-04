//! Defines a template parameter class.

use std::{borrow::Cow, fmt::Display};

type StringValue = Cow<'static, str>;

trait AutoParam {
    fn extend_params(self, params: &mut Vec<TemplateParameter>);
}
impl<P: IntoIterator<Item = TemplateParameter>> AutoParam for P {
    fn extend_params(self, params: &mut Vec<TemplateParameter>) {
        params.extend(self.into_iter());
    }
}
impl AutoParam for TemplateParameter {
    fn extend_params(self, params: &mut Vec<TemplateParameter>) {
        params.push(self);
    }
}

#[derive(Debug, PartialEq)]
/// Representation of a wikitext template.
pub struct Template {
    name: StringValue,
    params: Vec<TemplateParameter>,
}
impl Template {
    pub fn new<T: Into<StringValue>>(name: T, params: Vec<TemplateParameter>) -> Self {
        Self {
            name: name.into(),
            params,
        }
    }

    pub fn named<T: Into<StringValue>>(name: T) -> Self {
        Self::new(name, vec![])
    }

    pub fn add_params<P: AutoParam>(mut self, params: P) -> Self {
        params.extend_params(&mut self.params);
        self
    }
}
impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{{")?;
        write!(f, "{}", self.name)?;

        for param in self.params.iter() {
            f.write_str("\n")?;
            Display::fmt(param, f)?;
        }

        f.write_str("\n}}")
    }
}

#[derive(Debug, PartialEq)]
/// Representation of a wikitext template parameter.
pub struct TemplateParameter {
    key: StringValue,
    value: StringValue,
}
impl TemplateParameter {
    /// Create a parameter.
    pub fn new<T, U>(key: T, value: U) -> Self
    where
        T: Into<StringValue>,
        U: Into<StringValue>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
impl Display for TemplateParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{key} = {value}", key = self.key, value = self.value)
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
        print(f'Some(TemplateParameter::new("{key}", "{value}"))')
    else:
        print(f'TemplateParameter::new("{key}", "{value}"),')

*/
