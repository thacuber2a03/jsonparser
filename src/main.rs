use std::{fs, process::exit};

fn main() -> std::io::Result<()> {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() <= 1 {
        println!("usage: {} <filename>", argv[0]);
        exit(-1);
    }

    let str = fs::read_to_string(&argv[1]).unwrap_or_else(|e| {
        println!("couldn't read \"{}\": {e}", argv[1]);
        exit(-1);
    });

    println!("{str}");

    Ok(())
}
