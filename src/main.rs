use std::io::{self, BufRead};
extern crate rusty_lisp;
use rusty_lisp::LispInterpreter;


fn main() {
    let mut interpreter = LispInterpreter::new();

    let program = r#"
        (def! a 6)
        a
        (+ a 5)
        (let* (c 2) c)
        (= 1 1)
        (if (= 1 2) (+ 1 2))
        (def! str "abc")
        (print str)
    "#;

    for line in program.trim().lines() {
        let res = interpreter.evaluate(line.trim());
        println!("{} => {}", line, res);
    }

    //REPL
    let stdin = io::stdin();

    loop {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let res = interpreter.evaluate(line.trim());
        println!("=> {}", res);
    }
}



