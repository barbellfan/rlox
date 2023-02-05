use std::fmt;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: String, line: i32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexeme: {}\nTokenType: {:?}\nLiteral: {}\nLine: {}\n", self.lexeme, self.token_type, self.literal, self.line)
    }
}

#[derive(Debug)]
#[allow(dead_code)] // temp. ignore dead code until these get used.
#[allow(non_camel_case_types)] // allow non-camel-case typs for tokens.
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF
}

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
        scan_token(&scanner);
    }

    scanner.tokens.push(Token::new(TokenType::EOF, String::new(), String::new(), 0));

    Ok(scanner.tokens)
}

fn is_at_end(scanner: &Scanner) -> bool {
    scanner.current >= scanner.source.len()
}

fn scan_token(_scanner: &Scanner) {
    // do something
}
