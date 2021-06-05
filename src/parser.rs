#![allow(dead_code)] // for now
#![allow(unused_variables)] // for now
#![allow(unused_mut)] // for now

mod types;
pub struct Token {
    token_type: types::Types,
    value: String
}

pub fn parse(file_contents: String) -> i32 {
    // split the input contents into sperate lines
    let mut token_list: Vec<Token> = Vec::new();
    let mut lines: Vec<String> = linefy(file_contents);

    for line in lines.iter() {
        println!("{}", line);
    }
    
    0
}

fn linefy(input: String) -> Vec<String> {
    let mut lineified: Vec<String> = Vec::new();
    let mut raw_lines = input.lines();
    for mut line in raw_lines {
        // ignore comments
        if !line.trim().starts_with("//") {
            lineified.push(line.trim().to_string());
        }
    }
    lineified
}