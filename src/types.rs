use std::collections::HashMap;

#[derive(Clone)]
pub enum LispDataType {
    Number(i32),
    BuiltInFunction(fn(Vec<LispDataType>) -> LispDataType)
}

pub struct Environment<'a> {
    map: Box<HashMap<&'a str, LispDataType>>,
    next: Option<Box<Environment<'a>>>
}

impl<'a> Environment<'a> {
    pub fn set(&'a mut self, name: &'a str, value: LispDataType) {
        self.map.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<LispDataType> {
        match self.map.get(name) {
            None if self.next.is_some() => {
                let boxed_next = self.next.as_ref();
                boxed_next.unwrap().get(name)
            },
            None => None,
            r => Some((*r.unwrap()).clone())
        }
    }
}

fn plus(args: Vec<LispDataType>) -> LispDataType {
    LispDataType::Number(args.iter().fold(0, |acc, n| match *n {
        LispDataType::Number(number) => acc + number,
        _ => panic!("Trying to add not number type")
    }))
}

fn minus(args: Vec<LispDataType>) -> LispDataType {
    LispDataType::Number(args.iter().fold(0, |acc, n| match *n {
        LispDataType::Number(number) => acc - number,
        _ => panic!("Trying to subtract not number type")
    }))
}

fn multiply(args: Vec<LispDataType>) -> LispDataType {
    LispDataType::Number(args.iter().fold(1, |acc, n| match *n {
        LispDataType::Number(number) => acc * number,
        _ => panic!("Trying to multiply not number type")
    }))
}

fn divide(args: Vec<LispDataType>) -> LispDataType {
    let first_value = match args.get(0) {
        Some(&LispDataType::Number(number)) => number,
        _ => 0
    };

    LispDataType::Number(args.iter().fold(first_value * first_value, |acc, n| match *n {
        LispDataType::Number(number) => acc / number,
        _ => panic!("Trying to divide not number type")
    }))
}


pub fn get_environment<'a>() -> Environment<'a> {
    let mut environment = Box::new(HashMap::new());
    environment.insert("+", LispDataType::BuiltInFunction(plus));
    environment.insert("-", LispDataType::BuiltInFunction(minus));
    environment.insert("*", LispDataType::BuiltInFunction(multiply));
    environment.insert("/", LispDataType::BuiltInFunction(divide));

    Environment { map: environment, next: None}
}