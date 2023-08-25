# a Rust noob writes a JSON parser

so you want to write a JSON parser in Rust? ...yeah, me too.
[I've already written one or two compilers before](https://github.com/thacuber2a03/fe2lua), so I know what I'm doing. sorta.

I started with a simple frontend:
```rust
use std::env;
use std::process;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() <= 1 {
        println!("usage: {} <filename>", argv[0]);
        process::exit(-1);
    }

    let s = fs::read_to_string(&argv[1]);
    if let Err(e) = s {
        println!("couldn't open file {}: {e}", argv[1]);
        return Err(e);
    }
    let s = s.unwrap();
    println!("{s}");

    Ok(())
}
```

now, onto the lexer:

```rust
use std::io::Read;

#[derive(Debug)]
pub struct Lexer<R> {
    input: R,
}

impl<R: Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        Lexer {
            input
        }
    }
}
```

not much going on over here right now. I added a loop in `main.rs` that goes through each token and prints it, just to test it.
I also switched to using a `BufReader`, as the lexer accepts anything that implements `Read`:

```rust
mod lexer;

use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::process;

use lexer::Lexer;

fn main() -> io::Result<()> {
    // ...

    let f = File::open(&argv[1])?;
    let r = BufReader::new(f);
    let lexer = Lexer::new(r);
    
    while let Some(token) = lexer.next() {
        println!("{token:?}");
    }

    Ok(())
}
```

I'm going to use an on-demand style lexer as that is much faster.
not exactly sure if in performance, but rather in usage: it works _and_ reads like an iterator.
I should probably also implement `Iterator` for `Lexer`...

anyways, gotta make a `Token` enum:

```rust
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
```

that wasn't hard, just a bit tedious.

and it's over here where I realized I _can_ implement `Iterator` for `Lexer`;
I basically already have the exact same structure as the trait, it's just adding the type alias and implementing `next`:

```rust
impl<R: Read> Iterator for Lexer<R> {
    type Item = R;
    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}
```

```rust
fn main() -> std::io::Result<()> {
    // ...

    for token in lexer {
        println!("{token:?}");
    }

    // ...
}
```

now I'll focus on implementing `Lexer::next`.
first, the single character tokens.

```rust
fn next(&mut self) -> Option<Self::Item> {
    match self.read_char() {
        Some(c) => Some(match c {
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ',' => Token::Comma,
            ':' => Token::Colon,
            c => panic!("unexpected character {c}"),
        }),
        None => None,
    }
}
```

they use this `read_char` function:
```rust
impl<R: Read> Lexer<R> {
    // ...

    fn read_char(&mut self) -> Option<char> {
        let mut buf = [0];
        if self.input.read(&mut buf).is_ok() {
            Some(buf[0] as char)
        } else {
            None
        }
    }
}
```

next, `true`, `false` and `null`:
```rust
fn next(&mut self) -> Option<Self::Item> {
    match self.read_char() {
        Some(c) => Some(match c {
            // ...
            't' | 'f' | 'n' => self.check_id(c),
            // ...
        }),
        // ...
    }
}
```

another helper method:
```rust
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
```

and now that we're dealing with errors more frequently,
I should probably implement an error method instead of just panicking...

first, I'd need to store the current position of the lexer:
```rust
#[derive(Debug)]
pub struct Lexer<R> {
    input: R,
    line: usize,
    col: usize,
}

impl<R: Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        Lexer {
            input,
            line: 1,
            col: 1,
        }
    }

    // ...
}
```

then, a simple method for reporting them and exiting gracefully (not exactly):
```rust
impl<R: Read> Lexer<R> {
    // ...

    fn error(&mut self, msg: String) -> ! {
        println!("error: {msg}");
        println!("at {}, {}", self.line, self.col);
        std::process::exit(-1);
    }

    // ...
}
```

I recently learned about `!`... it's apparently a type you return when your function or block or whatever isn't ever going to return.

it's different from `()` in that that is a valid type to be returned, but `!` just straight up says "you're never getting a value from me".

anyways, the `line` and `col` offsets are going to be changed in `Lexer::next`, while skipping whitespace.

I also conveniently forgot to skip whitespace, so there's that.

```rust
impl<R: Read> Iterator for Lexer<R> {
    // ...

    fn next(&mut self) -> Option<Self::Item> {
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
            // ...
        }
    }
}
```

now, replace all `panic!` calls with `self.error` calls.

```rust
fn check_id(&mut self, c: char) -> Token {
    // ...

    match s.as_str() {
        // ...
        _ => self.error(format!("expected true, false or null, got {s}")),
    }
}
```

```rust
impl<R: Read> Iterator for Lexer<R> {
    fn next(&mut self) -> Option<Token> {
        // ...

        match start {
            // ...
            c => self.error(format!("unexpected character {c}"))
        }
    }
}
```

up in this point, I realize I need a `Lexer::peek_char` method, as using `Lexer::read_char` is bound to fail if, for example, a `true` is scanned, as it can potentially skip another important character, such as `"` or `{`/`[`. no sane encoder usually outputs JSON without some whitespace, but there *are* such cases, and people happen to also write JSON files manually...

```rust
impl<R: Read> Lexer<R> {
    //...

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

    // ...
}
```
