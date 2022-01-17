pub mod types;
pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn lexer_test() {
        let o = tokenize(String::from("1+2*(sin(x)+b)/3"));
        println!("{:?}", o);
    }
}
