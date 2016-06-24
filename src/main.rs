use std::io::{self, BufRead};
extern crate rusty_lisp;
use rusty_lisp::LispInterpreter;


fn main() {
    let mut interpreter = LispInterpreter::new();

    //REPL
    println!("Rusty-Lisp REPL");
    let stdin = io::stdin();

    loop {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let res = interpreter.evaluate(line.trim());
        println!("=> {}", res);
    }
}



