use std::io::Read;

use crate::lexer::Lexer;

pub struct Parser<R> {
    lexer: Lexer<R>,
}

impl<R: Read> Parser<R> {
    pub fn new(lexer: Lexer<R>) -> Self {
        Parser {
            lexer,
        }
    }
}
