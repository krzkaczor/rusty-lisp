extern crate regex;

use std::rc::Rc;

mod ast;
mod parser;
mod types;

pub struct LispInterpreter {
    environment: types::Environment,
    verbose: bool
}

impl LispInterpreter {
    pub fn new() -> LispInterpreter {
        let environment = types::get_environment();
        LispInterpreter { environment: environment, verbose: false }
    }

    pub fn new_verbose() -> LispInterpreter {
        let environment = types::get_environment();
        LispInterpreter { environment: environment, verbose: true }
    }

    pub fn evaluate(&mut self, input: &str) -> i32 {
        let ast = parser::parse(input);

        let result = match ast {
            Some(ref ast) => ast.evaluate(&mut self.environment),
            None => Rc::new(types::LispDataType::Number(0))
        };

        let result = match *result {
            types::LispDataType::Number(result) => result,
            _ => panic!("Unknown result!")
        };

        if self.verbose {
            println!("{} -> {}", input, result);
        }

        result
    }

    pub fn evaluate_program(&mut self, program: &str) -> i32 {
        let mut last_result = 0;
        for line in program.lines() {
            last_result = self.evaluate(line.trim())
        }

        last_result
    }
}

#[cfg(test)]
mod tests {
    use super::LispInterpreter;

    #[test]
    fn it_works() {
        let mut interpreter = LispInterpreter::new();

        assert_eq!(0, interpreter.evaluate("(def! a 6)"));
        assert_eq!(6, interpreter.evaluate("a"));
        assert_eq!(11, interpreter.evaluate("(+ a 5)"));
        assert_eq!(36, interpreter.evaluate("(* a a)"));
        assert_eq!(2, interpreter.evaluate("(let* (c 2) c)"));
    }
}