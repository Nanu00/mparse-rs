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
    Sub,
    Add,
    Mul,
    Div,
    Pow
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Sub => 2,
            Operator::Add => 2,
            Operator::Mul => 3,
            Operator::Div => 3,
            Operator::Pow => 4,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Operator::Sub => write!(f, "{}", "-"),
            Operator::Add => write!(f, "{}", "+"),
            Operator::Mul => write!(f, "{}", "*"),
            Operator::Div => write!(f, "{}", "/"),
            Operator::Pow => write!(f, "{}", "^"),
        }
    }
}

impl FromStr for Operator {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
       let o = match s {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "/" => Operator::Div,
            "*" => Operator::Mul,
            "^" => Operator::Pow,
            _ => {return Err(ParseError::UnknownToken(s.to_string()))},
        };
        Ok(o)
    }
}

#[derive(Debug)]
pub enum Bracket {
    PLeft,  // Parantheses, left
    PRight, // Parantheses, right
}

impl FromStr for Bracket {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = match s {
            "(" => Bracket::PLeft,
            ")" => Bracket::PRight,
            _ => {return Err(ParseError::UnknownToken(s.to_string()))}
        };
        Ok(b)
    }
}

#[derive(Debug)]
pub enum Node {
    Literal(f64),
    Variable(String),
    Operator(Operator),
    Bracket(Bracket),
}

impl FromStr for Node {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Node, ParseError> {
        let n: Node;

        if let Ok(b) = Bracket::from_str(s) {
            n = Node::Bracket(b);
        } else if let Ok(o) = Operator::from_str(s) {
            n = Node::Operator(o);
        } else if let Ok(m) = s.parse::<u64>() {
            n = Node::Literal(m as f64);
        } else if s.chars().all(char::is_alphabetic) {
            n = Node::Variable(s.to_string());
        } else {
            return Err(ParseError::UnknownToken(s.to_string()));
        }
        Ok(n)
    }
}
