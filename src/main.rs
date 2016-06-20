extern crate rusty_lisp;
use rusty_lisp::LispInterpreter;


fn main() {
    let program = r#"
        (def! a 6)
        a
        (+ a 5)
        (let* (c 2) c)
    "#;

    let mut interpreter = LispInterpreter::new_verbose();

    println!("Result: {}", interpreter.evaluate_program(program.trim()));
}



