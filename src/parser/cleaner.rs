fn splitter(string: String, delimiter: &str) -> Vec<String> {
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

pub fn clean(input: String) -> Vec<String> {
    if input.contains(" ") {
        splitter(input, " ")
    }
    if input.contains("..") {
        splitter(input, "..")
    }
    if input.contains(".") {
        splitter(input, ".")
    }
    if input.contains(",") {
        splitter(input, ",")
    }
    if input.contains("{") {
        splitter(input, "{")
    }
    if input.contains("}") {
        splitter(input, "}")
    }
    if input.contains("[") {
        splitter(input, "[")
    }
    if input.contains("]") {
        splitter(input, "]")
    }
    if input.contains("(") {
        splitter(input, "(")
    }
    if input.contains(")") {
        splitter(input, ")")
    }
    if input.contains("==") {
        splitter(input, "==")
    } 
    if input.contains("++") {
        splitter(input, "++")
    } 
    if input.contains("--") {
        splitter(input, "--")
    } 
}