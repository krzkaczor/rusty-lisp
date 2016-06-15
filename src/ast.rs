use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum LispDataType {
    Number(i32)
}

pub type Environment<'a> = HashMap<&'a str, LispDataType>;

pub trait LispSyntax : Debug {
    fn evaluate(&self, environment: &Environment) -> LispDataType;
}

pub type Syntax = Box<LispSyntax>;

#[derive(Debug)]
pub struct List {
    pub children: Box<Vec<Syntax>>
}

impl LispSyntax for List {
    fn evaluate(&self, _: &Environment) -> LispDataType {
        return LispDataType::Number(1);
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub id: String
}
impl LispSyntax for Symbol{
    fn evaluate(&self, env: &Environment) -> LispDataType {
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