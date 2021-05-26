fn spliter(string: String, delimiter: &str) -> Vec<String> {
    let mut split: Vec<String> = Vec::new();

    let u_index: usize = string.find(&delimiter).map(|string| string).unwrap();
    let (first, second) = string.split_at(u_index + delimiter.len());
    let holder_string: String = first.to_string();
    split.push(
        holder_string
            .split_at(holder_string.len() - delimiter.len())
            .0
            .to_string(),
    );
    split.push((&*delimiter).to_string());
    split.push(second.to_string());
    return split;
}

fn recursor(main_vec: &mut Vec<String>, helper: &mut Vec<String>, input: String, delimiter: &str) {
    main_vec.append(&mut spliter((&*input).to_string(), delimiter));
    for i in &mut main_vec.iter() {
        helper.append(&mut cleaner(i.to_string()));
    }
}

pub fn cleaner(input: String) -> Vec<String> {
    let mut cleaner_token_list: Vec<String> = Vec::new();
    let mut holder_token_list: Vec<String> = Vec::new();

    
    if input.len() > 2 {
        if input.contains("..") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), "..");
        } else if input.contains("::") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), "::");
        }
        cleaner_token_list.append(&mut holder_token_list);
        return cleaner_token_list;
    } else if input.len() > 1 {
        if input.contains(";") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), ";");
        } else if input.contains(",") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), ",");
        } else if input.contains(":") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), ":");
        } else if input.contains("{") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), "{");
        } else if input.contains("}") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), "}");
        } else if input.contains("(") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), "(");
        } else if input.contains(")") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), ")");
        } else if input.contains("[") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), "[");
        } else if input.contains("]") {
            recursor(&mut cleaner_token_list, &mut holder_token_list, (&*input).to_string(), "]");
        }
        cleaner_token_list.append(&mut holder_token_list);
        return cleaner_token_list;
    }

    cleaner_token_list.push((&*input).to_string());
    return cleaner_token_list;
}
