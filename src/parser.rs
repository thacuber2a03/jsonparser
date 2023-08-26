use std::io::Read;

use crate::lexer::Lexer;
use crate::value::Value;

pub struct Parser<R> {
    lexer: Lexer<R>,
}

impl<R: Read> Parser<R> {
    pub fn new(lexer: Lexer<R>) -> Self {
        Parser {
            lexer,
        }
    }

    pub fn parse(&mut self) -> Option<Value> {
    }
}
