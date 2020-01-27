use std::io;
use std::io::prelude::*;
use my_parser::Translate;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

#[macro_use] mod latex;
mod my_parser;
mod values;

fn main() {
    let mut input = String::new();
    for line in io::stdin().lock().lines() {
        input.push_str(line.unwrap().as_str());
        input.push('\n');
    }

    let result: String = parser::MainParser::new().parse(input.as_str())
        .unwrap()
        .eval_translate();

    println!("{}", result);

    let mut file = std::fs::File::create("demo.tex").expect("create failed");
    file.write_all(format!("{}\n{}\n{}", BEGIN!(), result, END!()).as_bytes())
        .expect("write failed");
}