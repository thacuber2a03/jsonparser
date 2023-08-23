# a Rust noob writes a JSON parser

so you want to write a JSON parser in Rust?
...yeah, me too.
[I've already written one or two compilers before](https://github.com/thacuber2a03/fe2lua), so I know what I'm doing. sorta.

I started with a simple frontend:
```rust
fn main() -> std::io::Result<()> {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() <= 1 {
        println!("usage: {} <filename>", argv[0]);
    }

    let str = std::fs::read_to_string(&argv[1]);
    if let Err(e) = str {
        println!("couldn't open file {}: {e}", argv[1]);
        return Err(e)
    }
    let str = str.unwrap();
    println!("{str}");

    Ok(())
}
```
I thought of using [?](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator) but then I wasn't going to be able to tell the user about any errors, so I went to matching instead.
