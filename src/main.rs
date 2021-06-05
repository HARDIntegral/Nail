use std::env;
use std::fs::File;
use std::io::prelude::*;

mod parser;

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
    let extension: String = input_file.split_off(input_file.len() - 2);
    if !extension.contains("ns") {
        println!("IMPROPER FILE TYPE!");
        return;
    }
    input_file.push_str("ns");

    // Open the right file for reading
    let mut file = File::open(input_file).expect("CANNOT OPEN FILE!");
    let mut file_contents: String = String::new();
    file.read_to_string(&mut file_contents)
        .expect("CANNOT READ FILE!");

    //----------Generate Abstract Syntax Tree-------------------------------------//
    // Parse File
    if parser::parse(file_contents) != 0 {
        println!("PARSE FAILURE");
    }
}
