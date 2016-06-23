use std::fmt::Debug;
use std::rc::Rc;
use std::any::Any;


use types::*;

pub trait LispSyntax : Debug {
    fn evaluate<'a>(&'a self, environment: &mut Environment) -> Rc<LispDataType>;
    fn as_any(&self) -> &Any;
}

pub type Syntax = Box<LispSyntax>;

#[derive(Debug)]
pub struct List {
    pub children: Box<Vec<Syntax>>
}

impl LispSyntax for List {
    fn evaluate(&self, env: &mut Environment) -> Rc<LispDataType> {
        let (head, tail) = self.children.split_first().unwrap();

        let function_data_type = head.evaluate(env);

        match *function_data_type {
            LispDataType::BuiltInFunction(function) => function(env, tail),
            _ => panic!("List evaluation error!")
        }
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub id: String
}
impl LispSyntax for Symbol{
    fn evaluate<'a>(&'a self, env: &mut Environment) -> Rc<LispDataType> {
        match env.get(self.id.as_str()) {
            None => panic!("Symbol \"{}\" not found!", self.id.as_str()),
            Some(data_type) => data_type
        }
    }

    fn as_any(&self) -> &Any {
        self
    }
}


#[derive(Debug)]
pub struct Number {
    pub number: i32
}
impl LispSyntax for Number {
    fn evaluate(&self, _: &mut Environment) -> Rc<LispDataType> {
        Rc::new(LispDataType::Number(self.number))
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Debug)]
pub struct Chars {
    pub string: String
}
impl LispSyntax for Chars {
    fn evaluate(&self, _: &mut Environment) -> Rc<LispDataType> {
        Rc::new(LispDataType::String(self.string.clone()))
    }

    fn as_any(&self) -> &Any {
        self
    }
}