extern crate regex;
use std::collections::HashMap;

mod ast;
mod parser;

use ast::LispDataType;

fn plus(args: Vec<LispDataType>) -> LispDataType {
    let sum = args.iter().fold(0, |acc, n| match *n {
        LispDataType::Number(number) => acc + number,
        _ => panic!("Trying to add not number type")
    });

    LispDataType::Number(sum)
}

fn multiply(args: Vec<LispDataType>) -> LispDataType {
    let sum = args.iter().fold(1, |acc, n| match *n {
        LispDataType::Number(number) => acc * number,
        _ => panic!("Trying to multiply not number type")
    });

    LispDataType::Number(sum)
}

pub fn run(input: &str) {
    println!("Input: {}", input);

    let ast = parser::parse(input);
    println!("AST: {:?}", ast);

    let mut environment: ast::Environment = HashMap::new();

    environment.insert("+", ast::LispDataType::BuiltInFunction(plus));
    environment.insert("*", ast::LispDataType::BuiltInFunction(multiply));

    let result = match ast {
        Some(ref ast) => ast.evaluate(&environment),
        None => ast::LispDataType::Number(0)
    };

    match result {
        LispDataType::Number(result) => println!("Evaluating expression: {:?}", result),
        _ => panic!("Unknown result!")
    }
}