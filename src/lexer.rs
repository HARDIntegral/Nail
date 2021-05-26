mod input_cleaner;
mod token_enum;
mod tokenizer;

//#[derive(Copy, Clone)]
pub struct Token {
    type_token: token_enum::TokenTypes,
    value: String,
}

pub fn tokenizer(input_string: String) -> Vec<Token> {
    // TODO: create function to tokenize the input file
    let mut token_list: Vec<Token> = Vec::new();

    let mut raw_tokens: Vec<&str> = Vec::<&str>::new();
    let raw_lines = input_string.lines();
    for line in raw_lines {
        // Remove comment lines
        if !line.to_string().trim().starts_with("//") {
            raw_tokens.append(&mut line.trim().split_whitespace().collect());
        }
    }
    raw_tokens.retain(|x| *x != "");

    let mut token_intermediate: String;
    for raw_token in raw_tokens.iter() {
        token_intermediate = raw_token.to_string();
        for token in input_cleaner::cleaner((&*token_intermediate).to_string()).iter() {
            token_list.push(tokenizer::tokenize(token.to_string()));
        }
    }

    for node in token_list.iter() {
        println!("{:?}\t\t\t{}", node.type_token, node.value);
    }

    return token_list;
}
