#![allow(dead_code)]

mod lexer;
mod parser;
mod value;

use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::process;

use lexer::Lexer;
use parser::Parser;

fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() <= 1 {
        println!("usage: {} <filename>", argv[0]);
        process::exit(-1);
    }

    let f = File::open(&argv[1])?;
    let r = BufReader::new(f);
    let lexer = Lexer::new(r);
    let parser = Parser::new(lexer);
    println!("{}", parser.parse());

    Ok(())
}
