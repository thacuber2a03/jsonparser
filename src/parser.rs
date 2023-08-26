use std::io::Read;

use crate::lexer::{Lexer, Token};
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
        self.lexer.next().map(|t| match t {
            Token::LBrace => self.object(),
            Token::LBracket => self.array(),
            t => panic!("unexpected token {:?}", t),
        })
    }

    fn array(&mut self) -> Value {
        Value::Null
    }

    fn object(&mut self) -> Value {
        Value::Null
    }
}
