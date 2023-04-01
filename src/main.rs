use std::{env, fs};

fn main() {
    // Get CLI arguments
    let args: Vec<String> = env::args().collect();

    // Get argument value
    let file_value_index = args.iter().position(|v| v == "-file").unwrap() + 1;
    let sort_value_index = args.iter().position(|v| v == "-sort").unwrap() + 1;
    let file_value = &args[file_value_index];
    let sort_value = &args[sort_value_index];

    // Read File
    let content = fs::read_to_string(file_value).expect("Expected to read the file");
    
    println!("{content}")
}

// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
// cargo run -- -file ./src/data/medicalreports.txt -sort PatientID
