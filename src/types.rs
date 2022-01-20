use std::{
    fmt::{
        self,
        Display,
        Formatter,
    },
    str::FromStr,
    error::Error
};

#[derive(Debug)]
pub enum ParseError {
    UnknownToken(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnknownToken(s) => write!(f, "Unknown token: {}", s),
            _ => write!(f, "Error: {}", self)
        }
    }
}

impl Error for ParseError {}

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

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Operator::SUB => write!(f, "{}", "-"),
            Operator::ADD => write!(f, "{}", "+"),
            Operator::MUL => write!(f, "{}", "*"),
            Operator::DIV => write!(f, "{}", "/"),
            Operator::POW => write!(f, "{}", "^"),
        }
    }
}

impl FromStr for Operator {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
       let o = match s {
            "+" => Operator::ADD,
            "-" => Operator::SUB,
            "/" => Operator::DIV,
            "*" => Operator::MUL,
            "^" => Operator::POW,
            _ => {return Err(ParseError::UnknownToken(s.to_string()))},
        };
        Ok(o)
    }
}

#[derive(Debug)]
pub enum Bracket {
    PLEFT,  // Parantheses, left
    PRIGHT, // Parantheses, right
}

impl FromStr for Bracket {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = match s {
            "(" => Bracket::PLEFT,
            ")" => Bracket::PRIGHT,
            _ => {return Err(ParseError::UnknownToken(s.to_string()))}
        };
        Ok(b)
    }
}

#[derive(Debug)]
pub enum Node {
    NUMBER(f64),
    VARIABLE(String),
    OPERATOR(Operator),
    BRACKET(Bracket),
}

impl FromStr for Node {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Node, ParseError> {
        let n: Node;

        if let Ok(b) = Bracket::from_str(s) {
            n = Node::BRACKET(b);
        } else if let Ok(o) = Operator::from_str(s) {
            n = Node::OPERATOR(o);
        } else if let Ok(m) = s.parse::<u64>() {
            n = Node::NUMBER(m as f64);
        } else if s.chars().all(char::is_alphabetic) {
            n = Node::VARIABLE(s.to_string());
        } else {
            return Err(ParseError::UnknownToken(s.to_string()));
        }
        Ok(n)
    }
}

pub struct Leaf {
    n: Node,
    leaves: Vec<Option<Leaf>>
}
