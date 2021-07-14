mod cleaner;
use cleaner::clean;
use super::types::Types;

//#![derive(Debug)]
pub struct Token {
    tok_type: Types,
    contents: String,
}

//#![derive(Copy, Clone, Debug)]
pub struct Parser {
    input: String
}

impl Parser {
    pub fn new(original_input: String) -> Parser {
        assert!(!original_input.is_empty());
        Parser { input: original_input}
    }
    //pub fn tokenize(&mut self) -> Vec<String> {
        
    //}

    pub fn linefy(&self) -> Vec<String> {
        let mut lineified: Vec<String> = Vec::new();
        let mut raw_lines = self.input.lines();
        for mut line in raw_lines {
            // ignore comments
            if !line.trim().starts_with("//") {
                lineified.push(line.trim().to_string());
            }
        }
        lineified
    }

    pub fn tokenize(&self, line: &str) -> Vec<String> {
        let mut tokens: Vec<String> = clean(line.to_string());
        let mut lexd: Vec<Token> = Vec::new();

        

        tokens
    }
}
