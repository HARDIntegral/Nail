mod token_enum;
mod tokenizer;

#[derive(Copy, Clone)]
pub struct Token {
    type_token: token_enum::TokenTypes,
    value: f32,
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
        let mut add_eol: bool = false;
        let mut add_com_sep: bool = false;
        let mut add_col_sep: bool = false;
        let mut add_dcol_sep: bool = false;
        let mut add_after_dcol_sep: bool = false;
        let mut add_dper_sep: bool = false;
        let mut add_after_dper_sep: bool = false;

        let mut holder_token_1: String = String::new();
        let mut holder_token_2: String = String::new();
        let mut holder_token_3: String = String::new();
        token_intermediate = raw_token.to_string();
        if token_intermediate.len() > 1 {
            if token_intermediate.ends_with(";") {
                let (first, _second) = token_intermediate.split_at(token_intermediate.len() - 1);
                holder_token_1 = first.to_string();
                holder_token_2 = String::from(";");
                add_eol = true;
            } else if token_intermediate.ends_with(",") {
                let (first, _second) = token_intermediate.split_at(token_intermediate.len() - 1);
                holder_token_1 = first.to_string();
                holder_token_2 = String::from(",");
                add_com_sep = true;
            } else if token_intermediate.ends_with("::") {
                let (first, _second) = token_intermediate.split_at(token_intermediate.len() - 2);
                holder_token_1 = first.to_string();
                holder_token_2 = String::from("::");
                add_dcol_sep = true;
            } else if token_intermediate.contains("::") {
                let u_index: usize = token_intermediate
                    .find("::")
                    .map(|token_intermediate| token_intermediate)
                    .unwrap();
                let (first, second) = token_intermediate.split_at(u_index + 2);
                holder_token_1 = first.to_string();
                holder_token_1 = holder_token_1
                    .split_at(holder_token_1.len() - 2)
                    .0
                    .to_string();
                holder_token_3 = second.to_string();
                holder_token_2 = String::from("::");
                add_dcol_sep = true;
                add_after_dcol_sep = true;
            } else if token_intermediate.ends_with("..") {
                let (first, _second) = token_intermediate.split_at(token_intermediate.len() - 2);
                holder_token_1 = first.to_string();
                holder_token_2 = String::from("..");
                add_dper_sep = true;
            } else if token_intermediate.contains("..") {
                let u_index: usize = token_intermediate
                    .find("..")
                    .map(|token_intermediate| token_intermediate)
                    .unwrap();
                let (first, second) = token_intermediate.split_at(u_index + 2);
                holder_token_1 = first.to_string();
                holder_token_1 = holder_token_1
                    .split_at(holder_token_1.len() - 2)
                    .0
                    .to_string();
                holder_token_3 = second.to_string();
                holder_token_2 = String::from("..");
                add_dper_sep = true;
                add_after_dper_sep = true;
            } else if token_intermediate.ends_with(":") {
                let (first, _second) = token_intermediate.split_at(token_intermediate.len() - 1);
                holder_token_1 = first.to_string();
                holder_token_2 = String::from(":");
                add_col_sep = true;
            } else {
                holder_token_1 = (&*token_intermediate).to_string();
            }
        } else {
            holder_token_1 = (&*token_intermediate).to_string();
        }

        let mut unique_name_register: Vec<f32> = Vec::new();
        token_list.push(tokenizer::tokenize(
            (&*holder_token_1).to_string(),
            &mut unique_name_register,
        ));
        if add_eol || add_com_sep || add_col_sep || add_dcol_sep || add_dper_sep {
            token_list.push(tokenizer::tokenize(
                (&*holder_token_2).to_string(),
                &mut unique_name_register,
            ))
        }
        if add_after_dcol_sep || add_after_dper_sep {
            token_list.push(tokenizer::tokenize(
                (&*holder_token_3).to_string(),
                &mut unique_name_register,
            ))
        }
    }

    for node in token_list.iter() {
        println!("{:?} {}", node.type_token, node.value);
    }

    return token_list;
}
