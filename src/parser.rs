#![allow(dead_code)]
#![allow(unused_variables)]

mod lexer;

pub fn lexical_analysis(file_contents: String) -> i32 {
    let lex_err = lexer::lexical_analysis(file_contents); 
    if lex_err != 0 { return lex_err; }

    0
}
