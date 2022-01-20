pub mod types;
pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn lexer_test() {
        let o = tokenize(String::from("1+2*(sin(x)+b)/3*β"));
        println!("{:?}", o);
        assert!(o.is_ok())
    }

    #[test]
    fn lexer_fail() {
        let o = tokenize(String::from("1+2*(sin(x)+b)/3%"));
        println!("{:?}", o);
        assert!(o.is_err());
    }
}
