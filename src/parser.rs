#![allow(dead_code)] // for now
#![allow(unused_variables)] // for now
#![allow(unused_mut)] // for now

mod types;
mod parse;

use parse::Parser;

pub struct Token {
    token_type: types::Types,
    value: String
}

pub fn parse(file_contents: String) -> i32 {
    // split the input contents into sperate lines
    let mut token_list: Vec<Token> = Vec::new();
    let mut parser: Parser = Parser::new(file_contents);

    let mut lines: Vec<String> = parser.linefy();

    let mut raw_tokens: Vec<String> = Vec::new();
    for mut line in lines.iter() {
        let mut tmp_tokenized_line: Vec<String> = parser.tokenize(line);
        raw_tokens.append(&mut tmp_tokenized_line);
    }

    for raw_token in raw_tokens.iter() {
        println!("{}", raw_token);
    }
    
    0
}

