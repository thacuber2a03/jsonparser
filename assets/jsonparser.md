---
name: jsonparser
title: a Rust noob writes a JSON parser
permalink: /jsonparser
---

# {{ page.title }}

hello again. if you're wondering why I'm welcoming you as if you had been in this site already, you should go check [the other part](/jsonlexer).

anyways, a JSON parser. doesn't seem that hard.

first I'll go back to the frontend and change some stuff:

{% highlight rust linenos %}
// ...

mod parser;

use parser::Parser;

fn main() -> io::Result<()> {
    // ...

    let parser = Parser::new(lexer);
    println!("{}", parser.parse());
    // remove the token loop if you come from the lexer article

    Ok(())
}
{% endhighlight %}

now, to the parser struct:

{% highlight rust linenos %}
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
        todo!();
    }
}
{% endhiglight %}

you might ask where did that `Value` enum come from?
well, we need *some* way of representing the result of parsing the file.

{% highlight rust linenos %}
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Null,
    Number(f32),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}
{% endhighlight %}

it's not too big. I thought on using `Option<Value>` everywhere, but that would just be tedious pattern matching at *best**.

I'll be mostly mirroring the spec here, so nothing very interesting will be going on.
