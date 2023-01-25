use std::env;
use std::io::Write;
use std::process;
use std::fs;
use std::io;

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
    let mut line = String::new();
    
    'outer: loop {
        line.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line == "\n" {
                    break 'outer;
                }
                run(line.bytes().collect()).unwrap();
            },
            Err(_) => {
                break 'outer;
            }
        }
    }
}

fn run(bytes: Vec<u8>) -> Result<(), std::io::Error> {
    dbg!("{}", bytes);

    Ok(())
}
