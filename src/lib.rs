extern crate regex;
use regex::Regex;
use std::string::String;

#[derive(Debug)]
pub enum TOKEN<'a> {
    Char(char),
    String(&'a str),
    SpecialChars(&'a str)
}

fn advance<'a>(rest_input: &'a str) -> (String, TOKEN<'a>) {
    let strings = Regex::new(r#"^"(?:\\.|[^\\"])*""#).unwrap();
    let special_character = Regex::new(r#"^[\[\]{}()'`~^@]"#).unwrap();
    let comment = Regex::new(r"^;.*").unwrap();
    let special_chars = Regex::new(r#"^[^\s\[\]{}('"`,;)]*"#).unwrap();


    let (to_skip, token) = if strings.is_match(rest_input) {
        let matched = strings.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), TOKEN::String(matched))
    } else if special_character.is_match(rest_input) {
        let matched = special_character.captures_iter(rest_input).next().unwrap().at(0).unwrap().to_string();
        (matched.len(), TOKEN::Char(matched.chars().next().unwrap()))
    } else if comment.is_match(rest_input) {
        let matched = comment.captures_iter(rest_input).next().unwrap().at(0).unwrap().to_string();
        (matched.len(), TOKEN::String("comment..."))
    } else if special_chars.is_match(rest_input) {
        let matched = special_chars.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), TOKEN::SpecialChars(matched))
    } else {
        panic!("UNRECOGNIZED INPUT!");
    };

    let advanced_input = rest_input.chars().skip(to_skip).collect();
    (advanced_input, token)
}

pub fn tokenize(input: String) {
    let mut rest_input: String = input.trim_left().to_string();
    println!("tokenizing: {}", rest_input);

    while rest_input.len() != 0 {
        {
            let rest_input_loc = {
                let (s, token) = advance(rest_input.as_str()); //@todo fix me! remove string
                println!("TOKEN {:?}", token);
                s
            };
            rest_input = rest_input_loc.to_string().trim_left().to_string();
        }
    }
    println!("tokenizing finished!")
}