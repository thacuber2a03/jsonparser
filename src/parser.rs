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
            stored: None,
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
        let mut v = vec![self.value()];

        while let &Some(Token::Comma) = self.peek() {
            match self.next() {
                Some(_) => (),
                None => panic!("what"),
            }
            v.push(self.value());
        }

        if let Some(Token::RBracket) = self.peek() {

        } else {
            panic!("expected right bracket at end of array");
        }

        Value::Array(v)
    }

    fn object(&mut self) -> Value {
        Value::Null
    }

    fn value(&mut self) -> Value {
        match self.next() {
            Some(t) => match t {
                Token::String(s) => Value::String(s),
                Token::Number(n) => Value::Number(n),
                Token::True => Value::Boolean(true),
                Token::False => Value::Boolean(false),
                Token::Null => Value::Null,
                t => panic!("unexpected token {t:?}"),
            }
            None => panic!("expected value"),
        }
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
