use crate::token::{lookup_ident, Token};

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        let mut lexer = Self {
            input: input.into().chars().collect(),
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some('=') if self.peek_char() == Some(&'=') => {
                self.read_char();
                Token::EQ
            }
            Some('=') => Token::ASSIGN,
            Some(';') => Token::SEMICOLON,
            Some('(') => Token::LPAREN,
            Some(')') => Token::RPAREN,
            Some(',') => Token::COMMA,
            Some('+') => Token::PLUS,
            Some('-') => Token::MINUS,
            Some('!') if self.peek_char() == Some(&'=') => {
                self.read_char();
                Token::NotEq
            }
            Some('!') => Token::BANG,
            Some('/') => Token::SLASH,
            Some('*') => Token::ASTERISK,
            Some('<') => Token::LT,
            Some('>') => Token::GT,
            Some('{') => Token::LBRACE,
            Some('}') => Token::RBRACE,
            Some('a'..='z') => return lookup_ident(&self.read_identifier()),
            Some('0'..='9') => return Token::INT(self.read_number().parse().unwrap_or_default()),
            Some(_) => Token::ILLEGAL,
            None => Token::EOF,
        };

        self.read_char();

        token
    }

    fn read_char(&mut self) {
        self.ch = if self.input.is_empty() {
            None
        } else {
            Some(self.input.remove(0))
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> Option<&char> {
        self.input.first()
    }

    fn read_identifier(&mut self) -> String {
        let mut output = String::new();
        while matches!(self.ch, Some('a'..='z')) {
            output += &self.ch.unwrap().to_string();
            self.read_char();
        }

        output
    }

    fn read_number(&mut self) -> String {
        let mut output = String::new();
        while matches!(self.ch, Some('0'..='9')) {
            output += &self.ch.unwrap().to_string();
            self.read_char();
        }

        output
    }

    fn skip_whitespace(&mut self) {
        while self.ch.map(|ch| ch.is_whitespace()).unwrap_or(false) {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::*;

    #[test]
    fn test_next_token() {
        const INPUT: &str = "=+(){},;";

        let tests = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
        ];

        let mut l = Lexer::new(INPUT);

        tests.into_iter().for_each(|token| {
            let tok = l.next_token();

            assert_eq!(token, tok);
        })
    }

    #[test]
    fn test_next_token_numbers_and_functions() {
        const INPUT: &str = "let five = 5;

let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";

        let tests = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(10),
            Token::EQ,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(10),
            Token::NotEq,
            Token::INT(9),
            Token::SEMICOLON,
            Token::EOF,
        ];

        let mut l = Lexer::new(INPUT);

        tests.into_iter().for_each(|token| {
            let tok = l.next_token();

            assert_eq!(token, tok);
        })
    }
}
