use std::env;
use std::process;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        let path = &args[1];
        run_file(path).unwrap();
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) -> Result<(), std::io::Error> {
    let bytes = fs::read(path).unwrap();
    run(bytes)
}

fn run_prompt() {
    println!("running prompt");
}

fn run(bytes: Vec<u8>) -> Result<(), std::io::Error> {
    dbg!("{}", bytes);

    Ok(())
}