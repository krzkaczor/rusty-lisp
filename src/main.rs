extern crate rusty_lisp;
use rusty_lisp::tokenize;


fn main() {
    let input = r#" ( 123 456 abc )"#.to_string();
    tokenize(input)
}

