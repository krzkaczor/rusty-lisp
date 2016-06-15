extern crate regex;
use regex::Regex;
use std::option::Option;
use std::iter::*;
use std::fmt::Debug;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Char(char),
    String(&'a str),
    SpecialChars(&'a str)
}

//@todo refactor. Regex should be staticly allocated.
fn advance(rest_input: &str) -> (usize, Option<Token>) {
    let white_characters = Regex::new(r"^([\s,])+").unwrap();
    let strings = Regex::new(r#"^"(?:\\.|[^\\"])*""#).unwrap();
    let special_character = Regex::new(r#"^[\[\]{}()'`~^@]"#).unwrap();
    let comment = Regex::new(r"^;.*").unwrap();
    let special_chars = Regex::new(r#"^[^\s\[\]{}('"`,;)]*"#).unwrap();

    if white_characters.is_match(rest_input) {
        let matched = white_characters.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), None)
    } else if strings.is_match(rest_input) {
        let matched = strings.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), Some(Token::String(matched)))
    } else if special_character.is_match(rest_input) {
        let matched = special_character.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), Some(Token::Char(matched.chars().next().unwrap())))
    } else if comment.is_match(rest_input) {
        let matched = comment.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), None)
    } else if special_chars.is_match(rest_input) {
        let matched = special_chars.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), Some(Token::SpecialChars(matched)))
    } else {
        panic!("UNRECOGNIZED INPUT!");
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut rest_input = input;

    while rest_input.len() != 0 {
        {
            let (consumed, token) = advance(rest_input);
            rest_input = &rest_input[consumed..];

            if let Some(token) = token {
                tokens.push(token);
            }
        }
    }

    tokens
}

#[derive(Debug, Clone)]
enum LispDataType {
    Number(i32)
}

type Environment<'a> = HashMap<&'a str, LispDataType>;

trait LispSyntax : Debug {
    fn evaluate(&self, environment: &Environment) -> LispDataType;
}
type Syntax = Box<LispSyntax>;

#[derive(Debug)]
struct List {
    children: Box<Vec<Syntax>>
}

impl LispSyntax for List {
    fn evaluate(&self, _: &Environment) -> LispDataType {
        return LispDataType::Number(1);
    }
}

//@todo copying!
#[derive(Debug)]
struct Symbol {
    id: String
}
impl LispSyntax for Symbol{
    fn evaluate(&self, env: &Environment) -> LispDataType {
        env.get(self.id.as_str()).unwrap().clone()
    }
}


#[derive(Debug)]
struct Number {
    number: i32
}
impl LispSyntax for Number{
    fn evaluate(&self, _: &Environment) -> LispDataType {
        LispDataType::Number(self.number)
    }
}

fn read_list<'a, 'b, I>(reader: &'a mut Peekable<I>) -> Option<Box<LispSyntax>> where I: Iterator<Item=&'b Token<'b>> {
    let mut list: Vec<Syntax> = Vec::new();

    loop {
        let should_end: bool = Some(&&Token::Char(')')) == reader.peek();

        if should_end {
            reader.next();
            break;
        } else {
            let form: Option<Box<LispSyntax>> = read_form(reader);
            if form.is_some() {
                list.push(form.unwrap());
            }
        }
    }

    Some(Box::new(List{children: Box::new(list)}))
}

fn read_atom<'a, 'b, I>(reader: &'a mut Peekable<I>) -> Option<Box<LispSyntax>> where I: Iterator<Item=&'b Token<'b>> {
    let current_atom = reader.next();

    let symbol_regex = Regex::new("^[:alpha:]+$").unwrap();

    match current_atom {
        Some(&Token::SpecialChars(chars)) if symbol_regex.is_match(chars) => Some(Box::new(Symbol{id: chars.to_string()})),
        Some(&Token::SpecialChars(chars)) => Some(Box::new(Number{number: (chars.parse::<i32>()).unwrap()})),
        _ => None
    }
}

fn read_form<'a, 'b, I>(reader: &'a mut Peekable<I>) -> Option<Box<LispSyntax>> where I: Iterator<Item=&'b Token<'b>> {
    let is_list: bool =  Some(&&Token::Char('(')) == reader.peek();

    match is_list {
        true => {
            reader.next();
            read_list(reader)
        },
        false => read_atom(reader)
    }
}

pub fn run(input: &str) {
    println!("Input: {}", input);

    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);

    let mut reader = tokens.iter().peekable();
    let lisp_ast = read_form(&mut reader);

    let environment: Environment = HashMap::new();

    let result = match lisp_ast {
        Some(ast) => ast.evaluate(&environment),
        None => LispDataType::Number(0)
    };

    println!("Evaluating expression: {:?}", result);
}