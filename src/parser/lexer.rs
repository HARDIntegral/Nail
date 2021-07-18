#![allow(dead_code)]
#![allow(unused_variables)]


use crate::types::*; 

fn to_lexeme(file_contents: String) -> Vec<char> {
    file_contents.chars().collect()
}

// expects a possible token and an extra character
fn matcher(candidate: String) -> types::Type {
    let shortened: &str = candidate.clone().pop().unwrap().to_string().as_str();
    let end_char: char = candidate.clone().chars().last().unwrap();
    match shortened {
        "import" => {}
        "from" => {}
        "struct" => {}
        "enum" => {}
        "typedef" => {}
        "fn" => {}
        "return" => {}
        "if" => {}
        "elif" => {}
        "else" => {}
        "for" => {}
        "while" => {}
        "in" => {}
        "range" => {}
        "break" => {}
        "continue" => {}
        "let" => {}
        "const" => {}
        "Num" => {}
        "char" => {}
        "bool" => {}
        "String" => {}
        "None" => {}
        "True" => {}
        "False" => {}
        "[" => {}
        "]" => {}
        "(" => {}
        ")" => {}
        "{" => {}
        "}" => {}
        "," => {}
        ".." => {}
        "." => {}
    }
}  

fn tokenize(lexemes: Vec<char>) -> (i32, Vec<Type>) {
    let mut tokens: Vec<Type> = Vec::new();
    
    // march through the lexemes to aquire tokens
    let mut lexeme_dump: String = String::new();
    

    (0, tokens)
}

pub fn lexical_analysis(file_contents: String) -> i32 {
    let mut lexemes: Vec<char> = to_lexeme(file_contents);
    let (tokenize_err, tokens) = tokenize(lexemes);
    if tokenize_err != 0 { return tokenize_err; }

    0
}
