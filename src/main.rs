extern crate rusty_lisp;
use rusty_lisp::read_str;


fn main() {
    let input = r#" ( 123 456 abc );comment"#;
    read_str(input)
}

