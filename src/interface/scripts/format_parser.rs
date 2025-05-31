//! Parses the page information format.

#[derive(Debug, PartialEq)]
/// Description of a [ParseNode].
pub enum ParseType {
    /// Variable (i.e. function call).
    Variable,
    /// Plain text, should be left as-is.
    Text,
}

#[derive(Debug)]
/// Node of the parse tree.
pub struct ParseNode<'a> {
    /// Content of node.
    pub content: &'a str,
    /// Type variant of node.
    pub ptype: ParseType,
}

/// Parses the string format into a Vec of ParseNodes.
// #[deprecated]
pub fn parse_info_format(format: &str) -> Vec<ParseNode> {
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

        let end = format.find('}').expect("Error: invalid page info format.");
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
