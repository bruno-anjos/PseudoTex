#[macro_use]
extern crate clap;
#[macro_use]
extern crate lalrpop_util;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

use clap::App;

use my_parser::Translate;

lalrpop_mod!(pub parser);

#[macro_use]
mod latex;
mod my_parser;

const INPUT: &str = "input";
const OUTPUT: &str = "output";

fn main() {
    let yaml = load_yaml!("cli/config.yml");

    let matches = App::from_yaml(yaml).get_matches();
    let mut string_read: String = String::new();

    if matches.is_present(INPUT) {
        string_read = read_input_file(matches.value_of(INPUT).unwrap()).unwrap();
    } else {
        read_from_stdin(&mut string_read);
        string_read = string_read.trim().to_string();
    }

    let result: String = parser::MainParser::new().parse(string_read.as_str())
        .unwrap()
        .eval_translate();

    let string_result: String = format!("{}\n{}\n{}", BEGIN!(), result, END!());

    if matches.is_present(OUTPUT) {
        write_to_file(matches.value_of(OUTPUT).unwrap(), &string_result);
    } else {
        println!("Result: {}", string_result);
    }
}

fn read_input_file(input_file_name: &str) -> Result<String> {
    let content = fs::read_to_string(input_file_name)?;
    Ok(content.trim().to_string())
}

fn read_from_stdin(file_content: &mut String) {
    let stdin = std::io::stdin();
    stdin
        .read_line(file_content)
        .expect("Error reading from stdin!");
}

fn write_to_file(file_name: &str, output: &str) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(output.as_bytes())
        .expect("Error writing to file!");
}