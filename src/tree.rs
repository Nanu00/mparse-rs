use std::fmt::Display;

use crate::types::*;

#[derive(Debug)]
pub struct Leaf {
    n: Node,
    leaves: Vec<Leaf>
}

impl Leaf {
    pub fn new(n: Node, leaves: Vec<Leaf>) -> Leaf {
        Leaf { n, leaves }
    }

    fn print_with_offset(&self, offset: usize) {
        for _ in 0..offset {
            print!("    ");
        }
        println!("{:?}", self.n);
        for l in self.leaves.iter() {
            l.print_with_offset(offset+1);
        }
    }

    pub fn print_as_tree(&self) {
        self.print_with_offset(0);
    }
}

impl From<Vec<Node>> for Leaf {
    fn from(tokens: Vec<Node>) -> Self {
        let mut out_stack: Vec<Leaf> = vec![];
        let mut op_stack: Vec<Node> = vec![];


        for node in tokens.into_iter() {
            match &node {
                Node::Literal(_) | Node::Variable(_) => {
                    out_stack.push(
                        Leaf {
                            n: node,
                            leaves: vec![],
                        }
                    )
                },
                Node::Operator(t_o) => {
                    while op_stack.len() > 0 {
                        if let Node::Bracket(_) = op_stack.last().unwrap() {
                            break;
                        } else if let Node::Operator(o) = op_stack.last().unwrap() {
                            if let Operator::Pow = t_o {
                                if t_o.precedence() <= o.precedence() {
                                    break;
                                }
                            } else if t_o.precedence() != o.precedence() {
                                break;
                            }
                        }

                        let mut new_l = Leaf::new(op_stack.pop().unwrap(), vec![out_stack.pop().unwrap(), out_stack.pop().unwrap()]);
                        new_l.leaves.reverse();
                        out_stack.push(new_l);
                    }
                    op_stack.push(node);
                },
                Node::Bracket(br) => {
                    if let Bracket::PLeft = br {
                        op_stack.push(node);
                    } else {
                        while op_stack.len() > 0 {
                            if let Node::Bracket(Bracket::PLeft) = op_stack.last().unwrap() {
                                op_stack.pop();
                                break;
                            } else {
                                let mut new_l = Leaf::new(op_stack.pop().unwrap(), vec![out_stack.pop().unwrap(), out_stack.pop().unwrap()]);
                                new_l.leaves.reverse();
                                out_stack.push(new_l);
                            }
                        }
                    }
                }
            }
        }

        while op_stack.len() > 0 {
            let mut new_l = Leaf::new(op_stack.pop().unwrap(), vec![out_stack.pop().unwrap(), out_stack.pop().unwrap()]);
            new_l.leaves.reverse();
            out_stack.push(new_l);
        }

        if let Some(l) = out_stack.pop() { l } else { Leaf::new(Node::Literal(0.0), vec![]) }
    }
}
