extern crate regex;
use regex::Regex;
use std::string::String;

fn advance(rest_input: String) -> String {
    let strings = Regex::new(r#"^"(?:\\.|[^\\"])*""#).unwrap();
    let special_character = Regex::new(r#"^[\[\]{}()'`~^@]"#).unwrap();
    let comment = Regex::new(r"^;.*").unwrap();
    let special_chars = Regex::new(r#"^[^\s\[\]{}('"`,;)]*"#).unwrap();


    let to_skip = if strings.is_match(rest_input.as_str()) {
        let matched = strings.captures_iter(rest_input.as_str()).next().unwrap().at(0).unwrap().to_string();
        println!("STRING TOKEN: {} length: {}", matched, matched.len());
        matched.len()
    } else if special_character.is_match(rest_input.as_str()) {
        let matched = special_character.captures_iter(rest_input.as_str()).next().unwrap().at(0).unwrap().to_string();
        println!("SPECIAL CHARACTER TOKEN: {} length: {}", matched, matched.len());
        matched.len()
    } else if comment.is_match(rest_input.as_str()) {
        let matched = comment.captures_iter(rest_input.as_str()).next().unwrap().at(0).unwrap().to_string();
        println!("COMMENT TOKEN: {} length: {}", matched, matched.len());
        matched.len()
    } else if special_chars.is_match(rest_input.as_str()) {
        let matched = special_chars.captures_iter(rest_input.as_str()).next().unwrap().at(0).unwrap().to_string();
        println!("SPECIAL CHARS TOKEN: {} length: {}", matched, matched.len());
        matched.len()
    } else {
        panic!("UNRECOGNIZED INPUT!");
    };

    rest_input.chars().skip(to_skip).collect()
}

pub fn tokenize(input: String) {
    let mut rest_input: String = input.trim_left().to_string();
    println!("tokenizing: {}", rest_input);

    while rest_input.len() != 0 {
        rest_input = advance(rest_input).trim_left().to_string();
    }
    println!("tokenizing finished!")
}