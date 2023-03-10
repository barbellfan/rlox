
mod token;

use token::{Token, TokenType};

pub struct Scanner {
    pub source: Vec<u8>, // Characters in file or typed in. Read as a u8 so it can be converted to char.
    tokens: Vec<Token>,
    start: usize, // Location in array. Use usize to match array index data type.
    current: usize, // ditto
    line: u32, // Line number. Use u32 since there can be lots of lines in a file.
    had_error: bool
}

#[derive(Debug, Clone)]
pub struct LoxError {
    line: u32,
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
            line: 1,
            had_error: false
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
    let c = advance(scanner);
    match c {
        '(' => add_token(scanner, TokenType::LEFT_PAREN, c.to_string()),
        ')' => add_token(scanner, TokenType::RIGHT_PAREN, c.to_string()),
        '{' => add_token(scanner, TokenType::LEFT_BRACE, c.to_string()),
        '}' => add_token(scanner, TokenType::RIGHT_BRACE, c.to_string()),
        ',' => add_token(scanner, TokenType::COMMA, c.to_string()),
        '.' => add_token(scanner, TokenType::DOT, c.to_string()),
        '-' => add_token(scanner, TokenType::MINUS, c.to_string()),
        '+' => add_token(scanner, TokenType::PLUS, c.to_string()),
        ';' => add_token(scanner, TokenType::SEMICOLON, c.to_string()),
        '*' => add_token(scanner, TokenType::STAR, c.to_string()),
        '!' if is_token_match(scanner, '=') => add_token(scanner, TokenType::BANG_EQUAL, "!=".to_string()),
        '!' => add_token(scanner, TokenType::BANG, c.to_string()),
        '=' if is_token_match(scanner, '=') => add_token(scanner, TokenType::EQUAL_EQUAL, "==".to_string()),
        '=' => add_token(scanner, TokenType::EQUAL, "=".to_string()),
        '<' if is_token_match(scanner, '=') => add_token(scanner, TokenType::LESS_EQUAL, "<=".to_string()),
        '<' => add_token(scanner, TokenType::LESS, "<".to_string()),
        '>' if is_token_match(scanner, '=') => add_token(scanner, TokenType::LESS_EQUAL, ">=".to_string()),
        '>' => add_token(scanner, TokenType::LESS, ">".to_string()),
        other => add_token(scanner, TokenType::STRING, other.to_string()),
        /* add later when all the other things get added to the match
        other => {
            scanner.had_error = true;
            let e = LoxError {
                line: scanner.line, 
                error_type: "Some kind of error".to_owned(), 
                error_message: format!("Incorrect token: {}", other)};
            e.report_error();
        }
        */
    }
}

fn advance(scanner: &mut Scanner) -> char {
    let c: char = scanner.source[scanner.current] as char;
    scanner.current += 1;
    c
}

fn is_token_match(scanner: &mut Scanner, expected: char) -> bool {
    if is_at_end(scanner) {
        return false;
    }
    if peek(scanner) == expected {
        advance(scanner);
        return true;
    }
    return false;
}

fn peek(scanner: &Scanner) -> char {
    scanner.source[scanner.current + 1] as char
}

fn add_token(scanner: &mut Scanner, token_type: TokenType, token: String) {
    let text = match std::str::from_utf8(&scanner.source[scanner.start .. scanner.current]){
        Ok(s) => s.to_owned(),
        Err(e) => panic!("Error at line {}: {}", scanner.line, e),
    };

    let t: Token = Token::new(token_type, text.to_owned(), token.into(), scanner.line);
    scanner.tokens.push(t);
}
