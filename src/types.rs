use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Operator {
    SUB,
    ADD,
    MUL,
    DIV,
    POW
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::SUB => 2,
            Operator::ADD => 2,
            Operator::MUL => 3,
            Operator::DIV => 3,
            Operator::POW => 4,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::SUB => write!(f, "{}", "-"),
            Operator::ADD => write!(f, "{}", "+"),
            Operator::MUL => write!(f, "{}", "*"),
            Operator::DIV => write!(f, "{}", "/"),
            Operator::POW => write!(f, "{}", "^"),
        }
    }
}

#[derive(Debug)]
pub enum Number {
    CONSTANT(f64),
    VARIABLE(String)
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::CONSTANT(n) => write!(f, "{}", n),
            Number::VARIABLE(v) => write!(f, "{}", v)
        }
    }
}

#[derive(Debug)]
pub enum Bracket {
    PLEFT,  // Parantheses, left
    PRIGHT, // Parantheses, right
}

#[derive(Debug)]
pub enum Node {
    NUMBER(Number),
    OPERATOR(Operator),
    BRACKET(Bracket)
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        let n: Node;

        lazy_static!{
            static ref P_LEFT: Regex = Regex::new(r"\(").unwrap();
            static ref P_RIGHT: Regex = Regex::new(r"\)").unwrap();
            static ref NUM: Regex = Regex::new(r"\d").unwrap();
            static ref OP: Regex = Regex::new("[+-/*^]").unwrap();
        };

        if P_LEFT.is_match(s) {
            n = Node::BRACKET(Bracket::PLEFT);
        } else if P_RIGHT.is_match(s) {
            n = Node::BRACKET(Bracket::PRIGHT);
        } else if NUM.is_match(s) {
            n = Node::NUMBER(Number::CONSTANT(s.parse::<f64>().unwrap()));
        } else if OP.is_match(s) {
            n = match s {
                "+" => Node::OPERATOR(Operator::ADD),
                "-" => Node::OPERATOR(Operator::SUB),
                "/" => Node::OPERATOR(Operator::DIV),
                "*" => Node::OPERATOR(Operator::MUL),
                "^" => Node::OPERATOR(Operator::POW),
                _ => Node::OPERATOR(Operator::MUL),
            };
        } else {
            n = Node::NUMBER(Number::VARIABLE(s.to_string()));
        }

        n
    }
}

pub struct Leaf {
    n: Node,
    leaves: Vec<Option<Leaf>>
}
