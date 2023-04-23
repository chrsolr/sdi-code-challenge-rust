use std::{
    env,
    fs::{self, File},
    io::{self, BufRead},
    path::Path, 
    fmt::{self, Formatter, Display}
};
use text_colorizer::*;

struct Arguments {
    filename: String,
    sort_by: String,
}

fn main() {
    let arguments = parse_args();

    // Read File
    // let content = fs::read_to_string(file_value).expect("Expected to read the file");
    split_records(arguments.filename);
}

// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
// cargo run -- --file ./src/data/medicalreports.txt --sort PatientID

fn split_records(filename: String) {
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


fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_how_to_use();
        eprintln!("{} wrong number of arguments: expect 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(1);
    }

    let file_value_index = args.iter().position(|v| v == "--file").unwrap() + 2;
    let sort_value_index = args.iter().position(|v| v == "--sort").unwrap() + 2;

    let filename = env::args().nth(file_value_index).expect("missing file: --file");
    let sort_by = env::args().nth(sort_value_index).expect("missin sort: --sort");


    Arguments { filename, sort_by }
}

fn print_how_to_use() -> () {
    eprintln!(
        "{} parse a file and sort by a given field",
        "SDI Challenge:".blue()
    );
    eprintln!(
        "Usage: {} {}",
        "SDI Challenge:".blue(),
        "--file <file location> --sort <sort field>".green()
    );
}