pub fn lex(bytes: &[u8]) -> Vec<Token> {
    let tokens = Vec::new();
    let mut cursor = 0;

    while cursor < bytes.len() {
        let ch = bytes[cursor] as char;

        if char::is_numeric(ch) {
            todo!(); // Implement Numbers
        }

        if char::is_alphabetic(ch) {
            todo!(); // Implement Token::Identifier
        }
        
        panic!("Unknown token '{} {}'", ch, ch as i16);
    }

    return tokens;
}

#[derive(Debug)]
pub enum TokenType {
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
    Identifier,
    Number,
    String,
    Comment,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub cursor: usize,
}

impl Token {
    pub fn new(token_type: TokenType, cursor: usize) -> Self {
        Self {
            token_type,
            cursor,
        }
    }
}