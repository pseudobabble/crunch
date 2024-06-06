use std::fs::File;
use std::io::Read;
use std::process;

pub mod interpreter;
pub mod parser;
pub mod types;

use interpreter::{Interpreter, Memory};
use parser::*;

fn main() -> () {
    let mut test_file = File::open("./test.cr").unwrap();
    let mut input_file_contents = String::new();
    test_file.read_to_string(&mut input_file_contents).unwrap();

    let mut program = vec![];
    for line in input_file_contents.lines().by_ref() {
        if line.is_empty() {
            continue;
        }
        let (input, parsed_line) = parse_line(line).unwrap();
        if !input.is_empty() {
            eprintln!("parsing error, input remaining {:?}", input);
            process::exit(1);
        }
        program.push(parsed_line);
    }

    let mut i: Memory = Interpreter::new(program.clone());

    i.run();

    println!("{:#?}", i.memory);

    ()
}
