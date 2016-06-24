extern crate regex;

use std::rc::Rc;

mod ast;
mod parser;
mod types;

pub struct LispInterpreter {
    environment: types::Environment
}

fn result_to_string(result: Rc<types::LispDataType>) -> String {
    match *result {
        types::LispDataType::Number(number) => number.to_string(),
        types::LispDataType::Boolean(true) => "t".to_string(),
        types::LispDataType::Boolean(false) => "f".to_string(),
        types::LispDataType::Void => "()".to_string(),
        types::LispDataType::String(ref str) => str.clone(),
        _ => panic!("Unknown result!")
    }
}

impl LispInterpreter {
    pub fn new() -> LispInterpreter {
        let environment = types::get_environment();
        LispInterpreter { environment: environment }
    }

    pub fn evaluate(&mut self, input: &str) -> String {
        let ast = parser::parse(input);

        let result = match ast {
            Some(ref ast) => ast.evaluate(&mut self.environment),
            None => Rc::new(types::LispDataType::Number(0))
        };

        let result_string = result_to_string(result);

        result_string
    }
}

#[cfg(test)]
mod tests {
    use super::LispInterpreter;

    #[test]
    fn it_works() {
        let mut interpreter = LispInterpreter::new();

        assert_eq!("0", interpreter.evaluate("(def! a 6)"));
        assert_eq!("6", interpreter.evaluate("a"));
        assert_eq!("11", interpreter.evaluate("(+ a 5)"));
        assert_eq!("36", interpreter.evaluate("(* a a)"));
        assert_eq!("2", interpreter.evaluate("(let* (c 2) c)"));
        assert_eq!("t", interpreter.evaluate("(= 4 4)"));
        assert_eq!("1", interpreter.evaluate("(if (= a 6) 1 2)"));
        assert_eq!("2", interpreter.evaluate("(if (= 1 2) 1 2)"));
        assert_eq!("()", interpreter.evaluate("(print (if (= a 6) \"True\" \"False\"))"));
    }

    #[test]
    fn string_eqality() {
        let mut interpreter = LispInterpreter::new();
        interpreter.evaluate("(def! a \"abc\")");
        assert_eq!("t", interpreter.evaluate("(= a \"abc\")"));
        assert_eq!("f", interpreter.evaluate("(= a \"abcd\")"));
    }

    #[test]
    fn symbol_case_insensitivity() {
        let mut interpreter = LispInterpreter::new();
        interpreter.evaluate("(def! someString \"abc\")");
        assert_eq!("t", interpreter.evaluate("(= somestring \"abc\")"));
        assert_eq!("t", interpreter.evaluate("(= someString \"abc\")"));
    }

    #[test]
    fn function_closures() {
        let mut interpreter = LispInterpreter::new();
        interpreter.evaluate("(def! id (fn* [a] a))");
        assert_eq!(r#""identity function""#, interpreter.evaluate(r#"(id "identity function")"#));
        assert_eq!(r#"5"#, interpreter.evaluate(r#"((fn* [a b] (+ a b)) 2 3)"#));
    }
}