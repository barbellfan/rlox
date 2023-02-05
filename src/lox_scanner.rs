
mod token;

use token::{Token, TokenType};

pub struct Scanner {
    pub source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

#[derive(Debug, Clone)]
pub struct LoxError {
    line: i64,
    error_type: String,
    error_message: String
}

impl LoxError {
    pub fn report_error(&self) {
        println!("[line {}] Error: {} : {}", self.line, self.error_type, self.error_message);
    }
}

impl Scanner {
    pub fn new(bytes: Vec<u8>) -> Self {
        Scanner {
            source: bytes,
            tokens: vec!(),
            start: 0,
            current: 0,
            line: 1
        }
    }
}

pub fn scan_tokens(mut scanner: Scanner) -> Result<Vec<Token>, LoxError> {
    while !is_at_end(&scanner) {
        scanner.start = scanner.current;
        scan_token(&mut scanner);
    }

    scanner.tokens.push(Token::new(TokenType::EOF, String::new(), String::new(), 0));

    Ok(scanner.tokens)
}

fn is_at_end(scanner: &Scanner) -> bool {
    scanner.current >= scanner.source.len()
}

fn scan_token(scanner: &mut Scanner) {
    scanner.current += 1;
}
