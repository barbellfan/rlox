
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
    let c = advance(scanner);
    match c {
        '(' => add_token(scanner, TokenType::LEFT_PAREN, c),
        ')' => add_token(scanner, TokenType::RIGHT_PAREN, c),
        '{' => add_token(scanner, TokenType::LEFT_BRACE, c),
        '}' => add_token(scanner, TokenType::RIGHT_BRACE, c),
        ',' => add_token(scanner, TokenType::COMMA, c),
        '.' => add_token(scanner, TokenType::DOT, c),
        '-' => add_token(scanner, TokenType::MINUS, c),
        '+' => add_token(scanner, TokenType::PLUS, c),
        ';' => add_token(scanner, TokenType::SEMICOLON, c),
        '*' => add_token(scanner, TokenType::STAR, c),
        other => println!("other token: {}", other)
    }
}

fn advance(scanner: &mut Scanner) -> char {
    let c: char = scanner.source[scanner.current] as char;
    scanner.current += 1;
    c
}

fn add_token(scanner: &mut Scanner, token_type: TokenType, token: char) {
    let text = match std::str::from_utf8(&scanner.source[scanner.start .. scanner.current]){
        Ok(s) => s.to_owned(),
        Err(e) => panic!("Error at line {}: {}", scanner.line, e),
    };

    let t: Token = Token::new(token_type, text.to_owned(), token.into(), 0);
    scanner.tokens.push(t);
}
