pub mod types;
pub mod lexer;
pub mod tree;

#[cfg(test)]
mod tests {
    use crate::{lexer::*, tree};

    #[test]
    fn lexer_test() {
        let o = tokenize(String::from("1+2*(sin(x)+b)/3*β"));
        // println!("{:?}", o);
        assert!(o.is_ok())
    }

    #[test]
    fn lexer_fail() {
        let o = tokenize(String::from("1+2*(sin(x)+b)/3%"));
        // println!("{:?}", o);
        assert!(o.is_err());
    }

    #[test]
    fn parser_test() {
        let t = tree::Leaf::from(tokenize(String::from("1+2*(a+b)/3*β")).unwrap());
        // println!("{:?}", t);
        t.print_as_tree();
    }
}
