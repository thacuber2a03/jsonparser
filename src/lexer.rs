use std::io::{Read, SeekFrom};

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
}

impl<R: Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        Lexer {
            input,
            line: 0,
            col: 0,
            stored: None,
        }
    }

    fn error(&mut self, msg: &str) {
        println!("error: {msg}");
        println!("at {}, {}", self.line, self.col);
        std::process::exit(-1);
    }

    fn read_char(&mut self) -> Option<char> {
        let mut buf = [0];
        if self.input.read(&mut buf).is_ok() {
            Some(buf[0] as char)
        } else {
            None
        }
    }

    fn check_id(&mut self, c: char) -> Token {
        let s = String::from(c);
        while let c = self.read_char() {
            match c {
                Some(c) => s.push(c),
                None => panic!("expected true, false or null, got {s}")
            }
        }

        match s.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            _ => panic!("expected true, false or null, got {s}")
        }
    }

    fn do_next() {
        let start: Option<char>;
        loop {
            match self.read_char() {
                Some(c) => {
                    match c {
                        ' ' | '\r' | '\t' => {
                            self.col += 1;
                            return self.next() // yay, recursion
                        }
                        '\n' => {
                            self.line += 1;
                            self.col = 1;
                            // yay, even more recursion
                            // (I really hope this is tail-call optimized :sweating:)
                            return self.next()
                        }
                        c => {
                            start = Some(c);
                            break
                        }
                    }
                },
                None => return None
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
                c => panic!("unexpected character {c}"),
            }),
            None => None,
        }
    }
}

impl<R: Read> Iterator for Lexer<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.do_next()
    }
}
