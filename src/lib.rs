extern crate regex;

mod ast;
mod parser;
mod types;

pub fn run(input: &str) {
    println!("Input: {}", input);

    let ast = parser::parse(input);
    println!("AST: {:?}", ast);

    let environment = types::get_environment();

    let result = match ast {
        Some(ref ast) => ast.evaluate(&environment),
        None => types::LispDataType::Number(0)
    };

    match result {
        types::LispDataType::Number(result) => println!("Evaluating expression: {:?}", result),
        _ => panic!("Unknown result!")
    }
}