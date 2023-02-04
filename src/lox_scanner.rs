use std::fmt;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32
}

impl Token {
    fn new(lexeme: String) -> Token {
        Token {
            token_type: TokenType::EOF,
            lexeme: lexeme,
            literal: "".to_owned(),
            line: 0
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexeme: {}\nTokenType: {:?}\nLiteral: {}\nLine: {}\n", self.lexeme, self.token_type, self.literal, self.line)
    }
}

#[derive(Debug)]
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
    tokens: Vec<Token>
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
            tokens: vec!()
        }
    }

    pub fn scan_tokens(scanner: Scanner) -> Result<Vec<Token>, LoxError> {
        // temporary code just to show that the function can read the scanner struct.
        let mut all_the_tokens: Vec<Token> = Vec::new();
        scanner.source.iter().for_each(|f| {
            let a_byte = *f;

            let a_string = String::from_utf8(vec!(a_byte)).unwrap();

            let token: Token = Token::new(a_string);
            all_the_tokens.push(token);
        });

        Ok(all_the_tokens)
    }
}
