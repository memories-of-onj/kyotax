#[derive(Debug, Clone, PartialEq)]
pub enum LitKind {
    Str {
        terminated: bool,
        raw_without_sign: String,
    },
    Char {
        terminated: bool,
        raw_without_sign: String,
    },
    Num,
    Bool,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommentKind {
    Line,
    Block { terminated: bool },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub raw: String,
    pub pos: usize,
}

impl Token {
    pub fn new<T>(kind: TokenKind, raw: T, pos: usize) -> Self
    where
        T: Into<String>,
    {
        Self {
            kind,
            raw: raw.into(),
            pos,
        }
    }

    pub fn is_nop(&self) -> bool {
        match self.kind {
            TokenKind::Comment { .. } => true,
            TokenKind::Whitespace => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdentKind {
    Ident,
    Raw { terminated: bool },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    And,
    Or,
    AndAnd,
    OrOr,
    ShiftL,
    ShiftR,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    BinOp(BinOpKind),
    BinOpEq(BinOpKind),
    Not,
    Eq,
    EqEq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    At,
    Dot,
    DotDot,
    DotDotEq,
    Comma,
    Semi,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Ident(IdentKind),
    Comment { kind: CommentKind, text: String },
    Whitespace,
    Let,
    Mut,
    Mod,
    Use,
    As,
    From,
    While,
    Loop,
    For,
    In,
    Async,
    Yield,
    Continue,
    Return,
    Break,
    Eof,
    Lit(LitKind),
    Fn,
    Enum,
    Pub,
    If,
    Else,
    Class,
    Extends,
}
