pub struct Token {
    pub lexeme: String
}

pub struct Scanner {
    pub source: Vec<u8>
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
    pub fn scan_tokens(scanner: Scanner) -> Result<Vec<Token>, LoxError> {
        // temporary code just to show that the function can read the scanner struct.
        let mut all_the_tokens: Vec<Token> = Vec::new();
        scanner.source.iter().for_each(|f| {
            let a_byte = *f;

            let a_string = String::from_utf8(vec!(a_byte)).unwrap();

            let token: Token = Token {
                lexeme: a_string
            };
            all_the_tokens.push(token);
        });

        Ok(all_the_tokens)
    }
}
