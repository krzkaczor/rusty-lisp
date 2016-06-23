extern crate regex;
use regex::Regex;
use std::option::Option;
use std::iter::*;
use ast::*;

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

fn read_list<'a, 'b, I>(reader: &'a mut Peekable<I>) -> Option<Box<LispSyntax>> where I: Iterator<Item = &'b Token<'b>> {
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

    Some(Box::new(List { children: Box::new(list) }))
}

fn read_atom<'a, 'b, I>(reader: &'a mut Peekable<I>) -> Option<Box<LispSyntax>> where I: Iterator<Item = &'b Token<'b>> {
    let current_atom = reader.next();

    let number_literal_regex = Regex::new("^[:digit:]+$").unwrap();

    match current_atom {
        Some(&Token::SpecialChars(chars)) if number_literal_regex.is_match(chars) => Some(Box::new(Number { number: (chars.parse::<i32>()).unwrap() })),
        Some(&Token::SpecialChars(chars)) => Some(Box::new(Symbol { id: chars.to_lowercase().to_string() })),
        Some(&Token::String(chars)) => Some(Box::new(Chars { string: String::from(chars) })),
        _ => None
    }
}

fn read_form<'a, 'b, I>(reader: &'a mut Peekable<I>) -> Option<Box<LispSyntax>> where I: Iterator<Item = &'b Token<'b>> {
    match reader.peek() {
        Some(&&Token::Char('(')) => {
            reader.next();
            read_list(reader)
        },
        _ => read_atom(reader)
    }
}

pub fn parse(input: &str) -> Option<Box<LispSyntax>> {
    let tokens = tokenize(input);

    let mut reader = tokens.iter().peekable();
    read_form(&mut reader)
}