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
        if !line.to_string().trim().starts_with("//") {
            // Remove comment lines
            raw_tokens.append(
                &mut line
                    .trim()
                    .split(['{', '(', ' ', ')', '}'].as_ref())
                    .collect(),
            );
        }
    }
    raw_tokens.retain(|x| *x != "");

    let mut token_intermediate: String;
    for raw_token in raw_tokens.iter() {
        let mut clean_tokens: Vec<String> = Vec::new();
        token_intermediate = raw_token.to_string();
        if token_intermediate.len() > 1 {
            if token_intermediate.ends_with(";") {
                let (first, _second) = token_intermediate.split_at(token_intermediate.len() - 1);
                clean_tokens.push(first.to_string());
                clean_tokens.push(String::from(";"));
            } else if token_intermediate.ends_with(",") {
                let (first, _second) = token_intermediate.split_at(token_intermediate.len() - 1);
                clean_tokens.push(first.to_string());
                clean_tokens.push(String::from(","));
            } else if token_intermediate.ends_with("::") {
                let (first, _second) = token_intermediate.split_at(token_intermediate.len() - 2);
                clean_tokens.push(first.to_string());
                clean_tokens.push(String::from("::"));
            } else if token_intermediate.contains("::") {
                let u_index: usize = token_intermediate
                    .find("::")
                    .map(|token_intermediate| token_intermediate)
                    .unwrap();
                let (first, second) = token_intermediate.split_at(u_index + 2);
                let holder_token: String = first.to_string();
                clean_tokens.push(holder_token.split_at(holder_token.len() - 2).0.to_string());
                clean_tokens.push(String::from("::"));
                clean_tokens.push(second.to_string());
            } else if token_intermediate.ends_with("..") {
                let (first, _second) = token_intermediate.split_at(token_intermediate.len() - 2);
                clean_tokens.push(first.to_string());
                clean_tokens.push(String::from(".."));
            } else if token_intermediate.contains("..") {
                let u_index: usize = token_intermediate
                    .find("..")
                    .map(|token_intermediate| token_intermediate)
                    .unwrap();
                let (first, second) = token_intermediate.split_at(u_index + 2);
                let holder_token: String = first.to_string();
                clean_tokens.push(holder_token.split_at(holder_token.len() - 2).0.to_string());
                clean_tokens.push(String::from(".."));
                clean_tokens.push(second.to_string());
            } else if token_intermediate.ends_with(":") {
                let (first, _second) = token_intermediate.split_at(token_intermediate.len() - 1);
                clean_tokens.push(first.to_string());
                clean_tokens.push(String::from(":"));
            } else {
                clean_tokens.push((&*token_intermediate).to_string());
            }
        } else {
            clean_tokens.push((&*token_intermediate).to_string());
        }

        for token in clean_tokens.iter() {
            token_list.push(tokenizer::tokenize(token.to_string()));
        }
    }

    for node in token_list.iter() {
        println!("{:?}\t\t\t{}", node.type_token, node.value);
    }

    return token_list;
}
