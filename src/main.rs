use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        let path = &args[1];
        run_file(path);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    println!("path: {}", path);
}

fn run_prompt() {
    println!("running prompt");
}