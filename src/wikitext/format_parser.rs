//! Parses the stage info format.

#![allow(dead_code, missing_docs)]
#[derive(Debug)]
pub enum ParseType {
    Variable,
    Text,
}
use ParseType::*;

#[derive(Debug)]
pub struct ParseObject<'a> {
    content: &'a str,
    ptype: ParseType,
}

pub fn parse_si_format(format: &str) -> Vec<ParseObject> {
    let mut parsed = vec![];
    let mut format = format;
    while let Some(mut n) = format.find("${") {
        if n != 0 {
            parsed.push(ParseObject {
                content: &format[0..n],
                ptype: Text,
            });
            format = &format[n..];
            n = 0;
        }
        let n = n;

        let end = format.find("}").expect("Invalid stage info format!");
        let var_name = &format[n + 2..end];
        parsed.push(ParseObject {
            content: var_name,
            ptype: Variable,
        });

        format = &format[end + 1..];
    }
    if !format.is_empty() {
        parsed.push(ParseObject {
            content: format,
            ptype: Text,
        });
    }

    parsed
}
