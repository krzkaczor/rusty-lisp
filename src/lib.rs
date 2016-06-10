extern crate regex;
use regex::Regex;
use std::option::Option;

#[derive(Debug)]
pub enum TOKEN<'a> {
    Char(char),
    String(&'a str),
    SpecialChars(&'a str)
}

fn advance(rest_input: &str) -> (usize, Option<TOKEN>) {
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
        (matched.len(), Some(TOKEN::String(matched)))
    } else if special_character.is_match(rest_input) {
        let matched = special_character.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), Some(TOKEN::Char(matched.chars().next().unwrap())))
    } else if comment.is_match(rest_input) {
        let matched = comment.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), None)
    } else if special_chars.is_match(rest_input) {
        let matched = special_chars.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), Some(TOKEN::SpecialChars(matched)))
    } else {
        panic!("UNRECOGNIZED INPUT!");
    }
}

pub fn tokenize(input: &str) {
    println!("tokenizing: {}", input);
    let mut rest_input = input;

    while rest_input.len() != 0 {
        {
            let (consumed, token) = advance(rest_input);
            rest_input = &rest_input[consumed..];

            if let Some(token) = token {
                println!("TOKEN {:?}", token);
            }
        }
    }
    println!("tokenizing finished!")
}