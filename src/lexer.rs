use unicode_segmentation::UnicodeSegmentation;

use crate::types::*;

pub fn tokenize(input: String) -> Vec<Node> {
    let mut output: Vec<Node> = Vec::new();

    println!("{}", &input);

    for s in input.split_word_bounds() {
        println!("{}", s);
        output.push(Node::from(s));
    }

    output
}
