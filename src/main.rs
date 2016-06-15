extern crate rusty_lisp;
use rusty_lisp::run;


fn main() {
    let input = r#" ( 123 456 abc );comment"#;
    run(input)
}

