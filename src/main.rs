use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

use scanner::Scanner;

mod scanner;
mod lox_token;

fn main() {
    let args: Vec<String> = env::args().collect();
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
    let mut line = String::new();
    
    loop {
        line.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line == "\n" {
                    break;
                }
                run(line.bytes().collect()).unwrap();
            },
            Err(_) => {
                break;
            }
        }
    }
}

fn run(bytes: Vec<u8>) -> Result<(), std::io::Error> {
    let scan = scanner::Scanner {
        source: bytes
    };

    let tokens = Scanner::scan_tokens(scan);

    tokens.iter().for_each(|f| println!("{}", f.lexeme));

    Ok(())
}
