use std::{
    env,
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

fn main() {
    // Get CLI arguments
    let args: Vec<String> = env::args().collect();

    // Get argument value
    let file_value_index = args.iter().position(|v| v == "-file").unwrap() + 1;
    let sort_value_index = args.iter().position(|v| v == "-sort").unwrap() + 1;
    let filename = &args[file_value_index];
    let _sort_value = &args[sort_value_index];

    // Read File
    // let content = fs::read_to_string(file_value).expect("Expected to read the file");

    // let a = read_file_lines(file_value);
    split_records(filename);
}

// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
// cargo run -- -file ./src/data/medicalreports.txt -sort PatientID

fn split_records(filename: &String) {
    if let Ok(lines) = read_file_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                // Skip empty lines
                if ip.trim().is_empty() {
                    continue;
                }

                if ip.starts_with("===") {
                    println!("END OF PATIENT RECORD: {}", ip);
                    continue;
                }
                
                println!("{}", ip)
            }
        }
    }
}

fn read_file_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
