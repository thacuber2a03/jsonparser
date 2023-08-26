use std::io::Read;

use crate::lexer::{Lexer, Token};
use crate::value::Value;

pub struct Parser<R> {
    stored: Option<Token>,
    lexer: Lexer<R>,
}

impl<R: Read> Parser<R> {
    pub fn new(lexer: Lexer<R>) -> Self {
        Parser {
            lexer,
        }
    }

    pub fn parse(&mut self) -> Option<Value> {
        self.next().map(|t| match t {
            Token::LBrace => self.object(),
            Token::LBracket => self.array(),
            t => panic!("unexpected token {:?}", t),
        })
    }

    fn array(&mut self) -> Value {
        let v = vec![self.value()];

        while let &Some(Token::Comma) = self.peek() {
            self.next();
        }

        Value::Array(v)
    }
    
    fn object(&mut self) -> Value {
        Value::Null
    }

    fn value(&mut self) -> Value {
        Value::Null
    }

    fn peek(&mut self) -> &Option<Token> {
        if self.stored.is_none() {
            self.stored = self.lexer.next();
        }
        &self.stored
    }

    fn next(&mut self) -> Option<Token> {
        if self.stored.is_some() {
            self.stored.take()
        } else {
            self.lexer.next()
        }
    }
}
