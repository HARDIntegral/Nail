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

pub fn clean(mut input: String) -> Vec<String> {
    println!("CLEANING");
    let mut tmp_holder: Vec<String> = Vec::new();

    if input.contains(" ") { tmp_holder.append(&mut splitter((&mut input).to_string(), " ")); } 
    if input.contains("..") { tmp_holder.append(&mut splitter((&mut input).to_string(), "..")); } 
    if input.contains(".") { tmp_holder.append(&mut splitter((&mut input).to_string(), ".")); } 
    if input.contains(",") { tmp_holder.append(&mut splitter((&mut input).to_string(), ",")); } 
    if input.contains("{") { tmp_holder.append(&mut splitter((&mut input).to_string(), "{")); } 
    if input.contains("}") { tmp_holder.append(&mut splitter((&mut input).to_string(), "}")); } 
    if input.contains("[") { tmp_holder.append(&mut splitter((&mut input).to_string(), "[")); } 
    if input.contains("]") { tmp_holder.append(&mut splitter((&mut input).to_string(), "]")); } 
    if input.contains("(") { tmp_holder.append(&mut splitter((&mut input).to_string(), "(")); } 
    if input.contains(")") { tmp_holder.append(&mut splitter((&mut input).to_string(), ")")); } 
    if input.contains("==") { tmp_holder.append(&mut splitter((&mut input).to_string(), "=")); } 
    if input.contains("++") { tmp_holder.append(&mut splitter((&mut input).to_string(), "++")); } 
    if input.contains("--") { tmp_holder.append(&mut splitter((&mut input).to_string(), "--")); } 
    else { tmp_holder.append(&mut vec!["".to_string()]); }

    tmp_holder
}