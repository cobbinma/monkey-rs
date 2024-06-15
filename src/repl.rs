use std::io::{self, BufRead, Write};

use crate::{lexer::Lexer, token::Token};

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        write!(out, "{}", PROMPT).unwrap();
        out.flush().unwrap();

        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();

        let mut l = Lexer::new(&line);
        loop {
            match l.next_token() {
                Token::EOF => break,
                token => writeln!(out, "{:?}", token).unwrap(),
            }
        }
    }
}
