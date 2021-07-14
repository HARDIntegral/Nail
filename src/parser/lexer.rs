#![allow(dead_code)]
#![allow(unused_variables)]

fn to_lexeme(file_contents: String) -> Vec<char> {
    file_contents.chars().collect()
}

fn tokenize(lexemes: Vec<char>) -> (i32, Vec<String>) {
    let mut tokens: Vec<String> = Vec::new();

    (0, tokens)
}

pub fn lexical_analysis(file_contents: String) -> i32 {
    let mut lexemes: Vec<char> = to_lexeme(file_contents);
    let (tokenize_err, tokens) = tokenize(lexemes);
    if tokenize_err != 0 { return tokenize_err; }

    0
}
