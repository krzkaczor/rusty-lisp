use std::collections::HashMap;
use std::collections::LinkedList;
use std::rc::Rc;
use ast::*;

pub enum LispDataType {
    Number(i32),
    BuiltInFunction(fn(&mut Environment, &[Syntax]) -> Rc<LispDataType>)
}

pub type Scope = HashMap<String, Rc<LispDataType>>;

pub struct Environment {
    scopes: LinkedList<Scope>,
}

impl Environment {
    pub fn new(root_environment: Scope) -> Environment {
        let mut scopes = LinkedList::new();
        scopes.push_back(root_environment);
        Environment { scopes: scopes }
    }

    pub fn set(&mut self, name: String, value: Rc<LispDataType>) {
        assert!(self.scopes.len() >= 1);
        self.scopes.front_mut().unwrap().insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Rc<LispDataType>> {
        for scope in &self.scopes {
            if let Some(value) = scope.get(name) {
                return Some((*value).clone());
            }
        }
        None
    }

    pub fn push_scope(&mut self) {
        self.scopes.push_front(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop_front();
    }
}

fn plus(env: &mut Environment, raw_args: &[Syntax]) -> Rc<LispDataType> {
    let args: Vec<Rc<LispDataType>> = raw_args.iter().map(|child| child.evaluate(env)).collect();

    Rc::new(LispDataType::Number(args.iter().fold(0, |acc, n| match **n {
        LispDataType::Number(number) => acc + number,
        _ => panic!("Trying to add not number type")
    })))
}

fn minus(env: &mut Environment, raw_args: &[Syntax]) -> Rc<LispDataType> {
    let args: Vec<Rc<LispDataType>> = raw_args.iter().map(|child| child.evaluate(env)).collect();

    Rc::new(LispDataType::Number(args.iter().fold(0, |acc, n| match **n {
        LispDataType::Number(number) => acc - number,
        _ => panic!("Trying to subtract not number type")
    })))
}

fn multiply(env: &mut Environment, raw_args: &[Syntax]) -> Rc<LispDataType> {
    let args: Vec<Rc<LispDataType>> = raw_args.iter().map(|child| child.evaluate(env)).collect();

    Rc::new(LispDataType::Number(args.iter().fold(1, |acc, n| match **n {
        LispDataType::Number(number) => acc * number,
        _ => panic!("Trying to multiply not number type")
    })))
}

fn divide(env: &mut Environment, raw_args: &[Syntax]) -> Rc<LispDataType> {
    let args: Vec<Rc<LispDataType>> = raw_args.iter().map(|child| child.evaluate(env)).collect();

    //todo: refactor
    let first_value = match args.get(0) {
        Some(value) => match **value {
            LispDataType::Number(number) => number,
            _ => 0
        },
        _ => 0
    };

    Rc::new(LispDataType::Number(args.iter().fold(first_value * first_value, |acc, n| match **n {
        LispDataType::Number(number) => acc / number,
        _ => panic!("Trying to divide not number type")
    })))
}

fn let_binding(env: &mut Environment, raw_args: &[Syntax]) -> Rc<LispDataType> {
    let mut raw_args_iterator = raw_args.iter();

    let list = match raw_args_iterator.next() {
        Some(ref value) => value.as_any().downcast_ref::<List>().expect("First argument for let* has to be list"),
        _ => panic!("Def at least two arguments!"),
    };

    let mut bindings_iterator = list.children.iter();
    let name = bindings_iterator.next().expect("has name").as_any().downcast_ref::<Symbol>().expect("Argument to be symbol");
    let value = bindings_iterator.next().expect("has value").as_any().downcast_ref::<Number>().expect("Argument to be number").evaluate(env);

    env.push_scope();
    env.set(name.id.to_string(), value);

    let result = raw_args_iterator.next().expect("has expression").evaluate(env);

    env.pop_scope();

    result
}

fn def(env: &mut Environment, raw_args: &[Syntax]) -> Rc<LispDataType> {
    let mut raw_args_iterator = raw_args.iter();

    let symbol_name = match raw_args_iterator.next() {
        Some(ref value) => value.as_any().downcast_ref::<Symbol>().expect("First argument for def! has to be symbol").id.clone(),
        _ => panic!("Def at least two arguments!"),
    };

    let args: Vec<Rc<LispDataType>> = raw_args_iterator.map(|child| child.evaluate(env)).collect();

    env.set(symbol_name, args.get(0).unwrap().clone());
    Rc::new(LispDataType::Number(0))
}
pub fn get_environment() -> Environment {
    let mut environment = HashMap::new();
    environment.insert("+".to_string(), Rc::new(LispDataType::BuiltInFunction(plus)));
    environment.insert("-".to_string(), Rc::new(LispDataType::BuiltInFunction(minus)));
    environment.insert("*".to_string(), Rc::new(LispDataType::BuiltInFunction(multiply)));
    environment.insert("/".to_string(), Rc::new(LispDataType::BuiltInFunction(divide)));
    environment.insert("def!".to_string(), Rc::new(LispDataType::BuiltInFunction(def)));
    environment.insert("let*".to_string(), Rc::new(LispDataType::BuiltInFunction(let_binding)));

    Environment::new(environment)
}