use std::io::Write;
use std::fs;
use std::fs::File;
use std::string::String;
use clap::{Arg, App};
use regex::Regex;
use lazy_static::lazy_static;
use itertools::Itertools;
// use itertools;

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

fn split_into_sentence(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\. ").unwrap();
    }
    let expanded: Vec<&str> = RE.split(s)
        .intersperse(". \n")
        .collect();
    let result: String = expanded.concat();
    result
}

fn main() {

    let matches = App::new("Combine_line")
        .version("1.1.0")
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
                let splitted_line: String = split_into_sentence(&one_line);
                result.push(splitted_line);
                temp = Vec::new();
            },
        }
    }

    if temp.len() > 0 {
        let one_line: String = temp.concat();
        let splitted_line: String = split_into_sentence(&one_line);
        result.push(splitted_line);
    }

    eprintln!("# lines after combining: {}", result.len());

    // write to file
    let mut file = match File::create(output_file) {
        Err(why) => panic!("Cannot create file {}: {}", output_file, why.to_string()),
        Ok(file) => file,
    };

    for item in result.into_iter().filter(|s| s.len() > 0) {
        writeln!(file, "{}", item).unwrap();
    }

}
