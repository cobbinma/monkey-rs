#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(i64),

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

pub fn lookup_ident(ident: &str) -> Token {
    dbg!(ident);
    match ident {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        non_keyword => Token::IDENT(non_keyword.to_string()),
    }
}
