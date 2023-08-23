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
