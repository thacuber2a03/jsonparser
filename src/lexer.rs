use std::{io::Read, mem};

#[derive(Debug)]
pub enum Token {
    True,
    False,
    Null,

    Comma,
    Colon,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Number(f32),
    String(String),
}

#[derive(Debug)]
pub struct Lexer<R> {
    input: R,
    line: usize,
    col: usize,
    stored: Option<Token>,
    stored_char: Option<char>,
}

impl<R: Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        Lexer {
            input,
            line: 0,
            col: 0,
            stored: None,
            stored_char: None,
        }
    }

    fn error(&mut self, msg: String) -> ! {
        println!("error: {msg}");
        println!("at {}, {}", self.line, self.col);
        std::process::exit(-1);
    }

    fn read_char(&mut self) -> Option<char> {
        if self.stored_char.is_some() {
            mem::replace(&mut self.stored_char, None)
        } else {
            let mut buf = [0];
            if self.input.read(&mut buf).is_ok() {
                Some(buf[0] as char)
            } else {
                None
            }
        }
    }

    fn peek_char(&mut self) -> &Option<char> {
        if self.stored_char.is_none() {
            self.stored_char = self.read_char()
        }
        &self.stored_char
    }

    fn check_id(&mut self, c: char) -> Token {
        let mut s = String::from(c);
        match self.peek_char() {
            Some(_) => s.push(self.read_char().unwrap()),
            None => self.error(format!("expected true, false or null, got {s}")),
        }

        match s.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            _ => self.error(format!("expected true, false or null, got {s}")),
        }
    }

    fn peek(&mut self) -> &Option<Token> {
        if self.stored.is_none() {
            self.stored = self.do_next();
        }
        &self.stored
    }

    fn do_next(&mut self) -> Option<Token> {
        let start: Option<char>;
        loop {
            match self.read_char() {
                Some(c) => {
                    match c {
                        ' ' | '\r' | '\t' => {
                            self.col += 1;
                            return self.next(); // yay, recursion
                        }
                        '\n' => {
                            self.line += 1;
                            self.col = 1;
                            // yay, even more recursion
                            // (I really hope this is tail-call optimized :sweating:)
                            return self.next();
                        }
                        c => {
                            start = Some(c);
                            break;
                        }
                    }
                }
                None => return None,
            }
        }

        match start {
            Some(c) => Some(match c {
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '[' => Token::LBracket,
                ']' => Token::RBracket,
                ',' => Token::Comma,
                ':' => Token::Colon,
                't' | 'f' | 'n' => self.check_id(c),
                c => self.error(format!("unexpected character {c}")),
            }),
            None => None,
        }
    }
}

impl<R: Read> Iterator for Lexer<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stored.is_some() {
            mem::replace(&mut self.stored, None)
        } else {
            self.do_next()
        }
    }
}
