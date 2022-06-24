use std::collections::HashMap;

#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftAngleBracket,
    RightAngleBracket,
    Plus,
    Minus,
    Asterisk,
    Backslash,
    Slash,
    Percent,
    GreaterThan,
    LessThan,
    DoubleEquals,
    Period,
    Comma,
    Equals,
    Semicolon,
    Colon,
    Underscore,
    And,
    Caret,
    Dollar,
    Hashtag,
    At,
    Exclamation,
    QuestionMark,
    Pipe,
    Identifier(String),
    Number(i16),
    String(String),
    Comment(String),
    EOF,
}

pub fn lex(src: &str) -> HashMap<Token, i16> {
    let tokens = HashMap::new();
    let cursor = 0;
    
    while cursor < src.len() {
        let ch = src.chars().nth(cursor).unwrap();

        if char::is_numeric(ch) {
            todo!(); // Implement Numbers
        }

        if char::is_alphabetic(ch) {
            todo!(); // Implement Token::Identifier
        }

        panic!("Unknown token '{:} {:}'", ch, ch as i16);
    }

    return tokens;
}
