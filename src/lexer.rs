use std::str::FromStr;

use unicode_segmentation::UnicodeSegmentation;

use crate::types::*;

pub fn tokenize(input: String) -> Result<Vec<Node>, crate::types::ParseError> {
    let mut output: Vec<Node> = Vec::new();

    for s in input.split_word_bounds() {
        let n = Node::from_str(s)?;
        output.push(n);
    }

    Ok(output)
}
