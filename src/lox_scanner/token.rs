use std::fmt;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: u32
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u32) -> Token {
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
        write!(
            f, 
            "Literal: {}\tLexeme: {}\tTokenType: {:?}\tLine: {}\n", 
            self.literal,
            self.lexeme,
            self.token_type,
            self.line)
    }
}

#[derive(Debug, PartialEq)]
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
