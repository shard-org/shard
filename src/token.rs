use crate::location::Span;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Symbols
    Ampersand,
    At,
    // Backtick,
    Backslash,
    Bang,
    Caret,
    Colon,
    Comma,
    Dollar,
    Dot,
    DoubleQuote,
    Equals,
    FatArrow,
    GreaterThan,
    GreaterThanEquals,
    LeftBrace,
    LeftBracket,
    LeftParen,
    LessThan,
    LessThanEquals,
    Minus,
    MinusMinus,
    NotEquals,
    Percent,
    Pipe,
    Plus,
    PlusPlus,
    Pound,
    Question,
    RightBrace,
    RightBracket,
    RightParen,
    Semicolon,
    SingleQuote,
    Slash,
    Star,
    Tilde,
    // TinyArrowLeft,
    TinyArrowRight,
    Underscore,

    // Other
    Ident(String),
    Register(RegSize, usize),
    WS,
    NL,

    // keywords
    Jmp,
    Ret,
    End,
    Init,
    Inline,
    Static,
    Const, 
    Entry,

    // literals
    BinLit(usize),
    CharLit(char),
    DecLit(usize),
    FloatLit(f64),
    HexLit(usize),
    OctLit(usize),
    StrLit(String),

    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RegSize {
//  Word,   // this will depend on architecture
    Double, // double as in double long.. not a double precision float
    Long,
    Short,
    HighByte,
    Byte,
}

impl Into<usize> for RegSize {
    fn into(self) -> usize {
        use RegSize::*;
        match self {
            Double   => 8,
            Long     => 4,
            Short    => 2,
            HighByte => 1,
            Byte     => 1,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token{kind,span}
    }
}
