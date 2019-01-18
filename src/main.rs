use std::io::Write;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::string::String;
use clap::{Arg, App};


fn last_char(s: &str) -> Option<char> {
    s.chars().rev().next()
}

fn is_end(s: &str) -> bool {
    let last = last_char(s.trim());
    match last {
        None => true,
        Some(v) => match v {
            '.' | ')' | '"' | '-' | ']' => true,
            _ => false,
        }
    }
}

fn main() {

    let matches = App::new("Combine_line")
        .version("0.10")
        .author("TK")
        .about("Combine multiple lines into one line.")
        .arg(Arg::with_name("input")
            .help("Name of the input file")
            .required(true)
            .index(1))
        .arg(Arg::with_name("output")
            .help("Name of the output file")
            .required(true)
            .index(2))
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();

    eprintln!("Converting {} -> {}", input_file, output_file);

    let content: String = fs::read_to_string(input_file).unwrap();
    let text_lines: Vec<&str> = content.lines().collect();
    eprintln!("# lines before combining: {}", text_lines.len());

    let mut result: Vec<String> = Vec::new();
    let mut temp: Vec<&str> = Vec::new();

    for line in content.lines() {
        temp.push(line);
        match is_end(line) {
            false => continue,
            true => {
                let one_line: String = temp.concat();
                result.push(one_line);
                temp = Vec::new();
            },
        }
    }

    if temp.len() > 0 {
        let one_line: String = temp.concat();
        result.push(one_line);
    }

    eprintln!("# lines after combining: {}", result.len());

    // write to file
    let mut file = match File::create(output_file) {
        Err(why) => panic!("Cannot create file {}: {}", output_file, why.description()),
        Ok(file) => file,
    };

    for item in result.iter() {
        writeln!(file, "{}", item).unwrap();
    }

}
