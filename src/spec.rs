use logos::{Lexer, Logos};

fn comment(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let txt = slice.split_at(1).1.trim(); // Trimming “#” and spaces
    Some(txt.to_string())
}

fn name(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_owned())
}

fn noop(_: &mut Lexer<Token>) -> () {
    ()
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex("[a-zA-Z0-9._-]*", name)]
    Name(String),

    #[regex("#(.*)", comment)]
    InlineComment(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
