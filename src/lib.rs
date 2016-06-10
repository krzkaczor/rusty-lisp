extern crate regex;
use regex::Regex;
use std::string::String;

#[derive(Debug)]
pub enum TOKEN {
    Char(char),
    String(String),
    SpecialChars(String)
}

fn advance(rest_input: &str) -> (String, TOKEN) {
    let strings = Regex::new(r#"^"(?:\\.|[^\\"])*""#).unwrap();
    let special_character = Regex::new(r#"^[\[\]{}()'`~^@]"#).unwrap();
    let comment = Regex::new(r"^;.*").unwrap();
    let special_chars = Regex::new(r#"^[^\s\[\]{}('"`,;)]*"#).unwrap();


    let (to_skip, token) = if strings.is_match(rest_input) {
        let matched = strings.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), TOKEN::String(matched.to_string()))
    } else if special_character.is_match(rest_input) {
        let matched = special_character.captures_iter(rest_input).next().unwrap().at(0).unwrap().to_string();
        (matched.len(), TOKEN::Char(matched.chars().next().unwrap()))
    } else if comment.is_match(rest_input) {
        let matched = comment.captures_iter(rest_input).next().unwrap().at(0).unwrap().to_string();
        (matched.len(), TOKEN::String("comment...".to_string()))
    } else if special_chars.is_match(rest_input) {
        let matched = special_chars.captures_iter(rest_input).next().unwrap().at(0).unwrap();
        (matched.len(), TOKEN::SpecialChars(matched.to_string()))
    } else {
        panic!("UNRECOGNIZED INPUT!");
    };

    let advanced_input = rest_input.chars().skip(to_skip).collect();
    (advanced_input, token)
}

pub fn tokenize(input: &str) {
    let mut rest_input:String = input.trim_left().to_string();
    println!("tokenizing: {}", rest_input);

    while rest_input.len() != 0 {
        {
            let (advanced_input, token) = advance(rest_input.as_str()); //@todo fix me! remove string
            rest_input = advanced_input.trim_left().to_string();
            println!("TOKEN {:?}", token);
//            let local_input = rest_input.clone();
//            let rest_input_loc = {
//

//                s
//            };
//            rest_input = rest_input_loc.trim_left();
        }
    }
    println!("tokenizing finished!")
}