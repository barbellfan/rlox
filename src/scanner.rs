use crate::lox_token::lox_token::Token;

pub(crate) struct Scanner {
    pub source: Vec<u8>
}

impl Scanner {
    pub fn scan_tokens(scanner: Scanner) -> Vec<Token> {
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

        all_the_tokens
    }
}
