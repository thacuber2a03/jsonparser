use std::io::Read;

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
            line: 1,
            col: 1,
            stored: None,
            stored_char: None,
        }
    }

    fn error(&mut self, msg: String) -> ! {
        println!("error: {msg}, at ({}, {})", self.line, self.col);
        std::process::exit(-1);
    }

    fn read_char(&mut self) -> Option<char> {
        let c = if self.stored_char.is_some() {
            self.stored_char.take().unwrap()
        } else {
            let mut buf = [0];
            if self.input.read(&mut buf).is_ok() {
                buf[0] as char
            } else {
                return None;
            }
        };

        match c {
            '\n' => {
                self.line += 1;
                self.col = 1;
            }
            _ => self.col += 1,
        }

        Some(c)
    }

    fn peek_char(&mut self) -> &Option<char> {
        if self.stored_char.is_none() {
            self.stored_char = self.read_char()
        }
        &self.stored_char
    }

    fn check_id(&mut self, c: char) -> Token {
        let mut s = String::from(c);
        loop {
            match self.peek_char() {
                Some(c) if !c.is_alphabetic() => break,
                Some(_) => s.push(self.read_char().unwrap()),
                None => self.error("expected true, false or null, got end of file".to_string()),
            }
        }

        match s.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            _ => self.error(format!("expected true, false or null, got {s}")),
        }
    }

    fn read_escape(&mut self) -> char {
        match self.read_char() {
            None => self.error("expected an escapable character, got end of file".to_string()),
            Some(c) => match c {
                '"' => '"',
                '\\' => '\\',
                '/' => '/', // what
                'b' => '\x08',
                'f' => '\x0c',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                'u' => {
                    // the hardest of them all
                    let mut s = String::new();
                    for _ in 0..4 {
                        match self.peek_char() {
                            None => {
                                self.error("expected a hex digits, got end of file".to_string())
                            }
                            Some(c) if c.is_ascii_hexdigit() => s.push(self.read_char().unwrap()),
                            Some(c) => {
                                let c = *c;
                                self.error(format!("expected a hex digit, got {c}"));
                            }
                        }
                    }

                    match s.parse::<char>() {
                        Ok(c) => c,
                        Err(e) => self.error(format!("{e}")), // should probably change this one
                    }
                }
                _ => self.error(format!("invalid escape character {c}")),
            },
        }
    }

    fn scan_string(&mut self) -> Token {
        let mut s = String::new();
        loop {
            match self.peek_char() {
                Some('"') => {
                    self.read_char();
                    break Token::String(s);
                }
                Some('\\') => {
                    self.read_char();
                    s.push(self.read_escape());
                }
                Some(_) => s.push(self.read_char().unwrap()),
                None => self.error("expected a closing quote, got end of file".to_string()),
            }
        }
    }

    fn scan_number(&mut self, n: char) -> Token {
        let mut s = String::from(n);
        while let Some(c) = self.peek_char() {
            match c {
                c if c.is_ascii_digit() => s.push(self.read_char().unwrap()),
                '-' | '+' | 'E' | 'e' | '.' => s.push(self.read_char().unwrap()),
                _ => break,
            }
        }

        if s.is_empty() {
            panic!("what");
        } else {
            match s.parse() {
                Ok(n) => Token::Number(n),
                Err(e) => self.error(e.to_string()),
            }
        }
    }
}

impl<R: Read> Iterator for Lexer<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            match self.peek_char() {
                Some(c) => match c {
                    c if c.is_ascii_whitespace() => self.read_char(),
                    _ => break,
                },
                None => return None,
            };
        }

        if self.peek_char().is_some_and(|c| c == '\0') {
            return None
        }

        self.read_char().map(|c| match c {
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ',' => Token::Comma,
            ':' => Token::Colon,
            't' | 'f' | 'n' => self.check_id(c),
            '"' => self.scan_string(),
            '-' => self.scan_number(c),
            c if c.is_ascii_digit() => self.scan_number(c),
            c => {
                self.error(format!("unexpected character {c} (code {})", c as u8))
            }
        })
    }
}
