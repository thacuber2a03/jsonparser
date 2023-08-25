#![allow(dead_code)]

mod lexer;

use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::process;

use lexer::Lexer;

fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() <= 1 {
        println!("usage: {} <filename>", argv[0]);
        process::exit(-1);
    }

    let f = File::open(&argv[1])?;
    let r = BufReader::new(f);
    let lexer = Lexer::new(r);

    for token in lexer {
        println!("{token:?}");
    }

    Ok(())
}
