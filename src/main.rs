use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;

mod lox_scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(Path::new(&args[1])).unwrap();
    } else {
        run_prompt();
    }
}

fn run_file(path: &Path) -> Result<(), std::io::Error> {
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
    let scan = lox_scanner::Scanner {
        source: bytes
    };

    let result = lox_scanner::Scanner::scan_tokens(scan);
    match result {
        Ok(tokens) => {
            tokens.iter().for_each(|f| print!("{}", f.lexeme));
        },
        Err(lox_error) => {
            lox_error.report_error();
        }
    }

    Ok(())
}
