mod spec;

use crate::spec::Token;
use logos::{Lexer, Logos};

fn comment(lex: &mut Lexer<Line>) -> Option<String> {
    let slice = lex.slice();
    let txt = slice.split_at(1).1.trim(); // Trimming “#” and spaces
    Some(txt.to_string())
}

fn spec(lex: &mut Lexer<Line>) -> Option<String> {
    let spec = lex.slice();
    let mut iter = Token::lexer(spec);
    while let Some(token) = iter.next() {
        println!("Spec: {:?}", token)
    }
    Some(format!("{}", ""))
}

#[derive(Logos, Debug, PartialEq)]
enum Line {
    #[regex("#(.*)", comment)]
    Comment(String),

    #[regex(r"[a-zA-Z0-9-._]+[ ]?(==|<)?[ ]?[.0-9]*[ ]?(#(.*))?", spec)]
    Spec(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn main() {
    let input = r#"
0._.-._.0
aiohttp
bpython  # for debug 
coverage==5.2.1
# LTS
django<3
    "#;

    for line in input.lines() {
        let mut lex = Line::lexer(line);

        while let Some(token) = lex.next() {
            if token != Line::Error {
                println!("{:?}", token)
            }
        }
    }
}
