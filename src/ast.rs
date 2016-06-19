use std::fmt::Debug;

use types::*;

pub trait LispSyntax : Debug {
    fn evaluate<'a>(&'a self, environment: &Environment<'a>) -> LispDataType;
}

pub type Syntax = Box<LispSyntax>;

#[derive(Debug)]
pub struct List {
    pub children: Box<Vec<Syntax>>
}

impl LispSyntax for List {
    fn evaluate(&self, env: &Environment) -> LispDataType {
        let evaluated: Vec<LispDataType> = self.children.iter().map(|child| child.evaluate(env)).collect();
        let (first, tail) = (evaluated.first(), evaluated[1..].to_vec());

        match first {
            Some(&LispDataType::BuiltInFunction(fun)) => fun(tail),
            _ => panic!("List evaluation error!")
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub id: String
}
impl LispSyntax for Symbol{
    fn evaluate<'a>(&'a self, env: &Environment<'a>) -> LispDataType {
        match env.get(self.id.as_str()) {
            None => panic!("Symbol {} not found!", self.id.as_str()),
            Some(data_type) => data_type.clone()
        }
    }
}


#[derive(Debug)]
pub struct Number {
    pub number: i32
}
impl LispSyntax for Number{
    fn evaluate(&self, _: &Environment) -> LispDataType {
        LispDataType::Number(self.number)
    }
}