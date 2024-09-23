//! Defines a template parameter class.

use std::io::Write;

pub struct TemplateParameter {
    key: &'static [u8],
    value: Vec<u8>,
}
impl TemplateParameter {
    pub fn new(key: &'static [u8], value: Vec<u8>) -> TemplateParameter {
        Self { key, value }
    }
}

impl From<TemplateParameter> for Vec<u8> {
    fn from(param: TemplateParameter) -> Self {
        let mut buf = vec![];
        for value in [b"|", param.key, b" = ", &param.value]{
            buf.write(value).unwrap();
        }
        buf
    }
}
