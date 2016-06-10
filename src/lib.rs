extern crate regex;
use regex::Regex;

#[derive(Debug)]
pub enum TOKEN<'a> {
    Char(char),
    String(&'a str),
    SpecialChars(&'a str)
}

fn advance(rest_input: &str) -> (usize, TOKEN) {
    let strings = Regex::new(r#"^"(?:\\.|[^\\"])*""#).unwrap();
    let special_character = Regex::new(r#"^[\[\]{}()'`~^@]"#).unwrap();
    let comment = Regex::new(r"^;.*").unwrap();
    let special_chars = Regex::new(r#"^[^\s\[\]{}('"`,;)]*"#).unwrap();


    if strings.is_match(rest_input) {
        let matched = strings.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), TOKEN::String(matched))
    } else if special_character.is_match(rest_input) {
        let matched = special_character.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), TOKEN::Char(matched.chars().next().unwrap()))
    } else if comment.is_match(rest_input) {
        let matched = comment.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), TOKEN::String("comment..."))
    } else if special_chars.is_match(rest_input) {
        let matched = special_chars.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), TOKEN::SpecialChars(matched))
    } else {
        panic!("UNRECOGNIZED INPUT!");
    }
}

pub fn tokenize(input: &str) {
    println!("tokenizing: {}", input);
    let mut rest_input = input.trim_left();

    while rest_input.len() != 0 {
        {
            let (consumed, token) = advance(rest_input);
//            println!("Move by {} bytes.", consumed);
            rest_input = &rest_input[consumed..].trim_left();
//            println!("Rest input size: {} bytes", rest_input.len());
            println!("TOKEN {:?}", token);
        }
    }
    println!("tokenizing finished!")
}