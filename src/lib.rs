extern crate regex;
use std::collections::HashMap;

mod ast;
mod parser;

pub fn run(input: &str) {
    let ast = parser::parse(input);

    let environment: ast::Environment = HashMap::new();

    let result = match ast {
        Some(ast) => ast.evaluate(&environment),
        None => ast::LispDataType::Number(0)
    };

    println!("Evaluating expression: {:?}", result);
}