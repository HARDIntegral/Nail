use std::env;
use std::fs::File;
use std::io::prelude::*;

mod lexer;

fn main() {
    //----------Verify and Configure file-----------------------------------------//
    // Handle the input
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("NO FILE GIVEN!");
        return;
    }

    // Determine if the file extension is correct
    let mut input_file: String = args[1].clone();
    let extension: String = input_file.split_off(input_file.len() - 3);
    if !extension.contains("nal") {
        println!("IMPROPER FILE TYPE!");
        return;
    }
    input_file.push_str("nal");

    // Open the right file for reading
    let mut file = File::open(input_file).expect("CANNOT OPEN FILE!");
    let mut file_contents: String = String::new();
    file.read_to_string(&mut file_contents)
        .expect("CANNOT READ FILE!");
    let end_of_file_extension: &str = "@@@";
    file_contents.push_str(end_of_file_extension);

    //----------Run Code in file--------------------------------------------------//
    // Test file reading
    lexer::tokenizer(file_contents);
}
