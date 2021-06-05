struct Parser {
    lines: Vec<String>
}

impl Parser {
    pub fn new(lines: Vec<String>) -> Parser {
        assert!(!lines.is_empty());
        Parser { lines }
    }
    pub fn tokenize(&mut self) -> Vec<String> {
        
    }
}