use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone)]
pub enum LispDataType {
    Number(i32),
    BuiltInFunction(fn(Vec<LispDataType>) -> LispDataType)
}

pub type Environment<'a> = HashMap<&'a str, LispDataType>;

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
        env.get(self.id.as_str()).unwrap().clone()
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