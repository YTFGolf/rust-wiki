//! Parses the stage info format.

#![allow(dead_code, missing_docs)]
#[derive(Debug, PartialEq)]
pub enum ParseType {
    Variable,
    Text,
}

#[derive(Debug)]
pub struct ParseNode<'a> {
    pub content: &'a str,
    pub ptype: ParseType,
}

pub fn parse_si_format(format: &str) -> Vec<ParseNode> {
    let mut parsed = vec![];
    let mut format = format;
    while let Some(mut n) = format.find("${") {
        if n != 0 {
            parsed.push(ParseNode {
                content: &format[0..n],
                ptype: ParseType::Text,
            });
            format = &format[n..];
            n = 0;
        }
        let n = n;

        let end = format.find("}").expect("Invalid stage info format!");
        let var_name = &format[n + 2..end];
        parsed.push(ParseNode {
            content: var_name,
            ptype: ParseType::Variable,
        });

        format = &format[end + 1..];
    }
    if !format.is_empty() {
        parsed.push(ParseNode {
            content: format,
            ptype: ParseType::Text,
        });
    }

    parsed
}
