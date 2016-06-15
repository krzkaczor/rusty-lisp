extern crate rusty_lisp;
use rusty_lisp::run;


fn main() {
    let input = r#" ( + (* 2 3) 1)"#;
    run(input)
}

