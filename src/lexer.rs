extern crate itertools;

use crate::error::{Error, Result};
use crate::scanner::Scanner;
use crate::token::{BinOpKind, CommentKind, IdentKind, LitKind, Token, TokenKind};
use itertools::Itertools;

pub struct Lexer {
    scanner: Scanner,
}

impl Lexer {
    pub fn new<T>(scanner: T) -> Self
    where
        T: Into<Scanner>,
    {
        Self {
            scanner: scanner.into(),
        }
    }

    fn read_by(&mut self, keywords: Vec<(&str, TokenKind)>) -> Result<Token> {
        let pos = self.scanner.get_pos();
        let lens = keywords
            .iter()
            .map(|(s, _)| s.len())
            .dedup()
            .sorted_unstable_by(|a, b| b.cmp(a));

        for len in lens {
            let raw = self.scanner.peek_len(len);

            match raw {
                Ok(raw) => {
                    for (s, kind) in &keywords {
                        if s == &raw {
                            self.scanner.skip_len(len)?;

                            return Ok(Token::new(kind.clone(), raw, pos));
                        }
                    }
                }
                Err(_) => continue,
            }
        }

        Err(Error::InvalidCharacter {
            chr: self.scanner.peek()?,
            pos,
        })
    }

    fn read_whitespace(&mut self) -> Result<Token> {
        let mut raw = String::new();
        let pos = self.scanner.get_pos();

        while !self.scanner.is_eof() && self.scanner.peek()?.is_whitespace() {
            raw += &self.scanner.read()?.to_string();
        }

        Ok(Token::new(TokenKind::Whitespace, raw, pos))
    }

    fn read_line_comment(&mut self) -> Result<Token> {
        let pos = self.scanner.get_pos();
        let mut raw = self.scanner.read_len(2)?;
        let mut text = String::new();

        while !self.scanner.is_eof() && self.scanner.peek()? != '\n' {
            let s = &self.scanner.read()?.to_string();

            raw += s;
            text += s;
        }

        Ok(Token::new(
            TokenKind::Comment {
                kind: CommentKind::Line,
                text,
            },
            raw,
            pos,
        ))
    }

    fn read_block_comment(&mut self) -> Result<Token> {
        let pos = self.scanner.get_pos();
        let mut raw = self.scanner.read_len(2)?;
        let mut text = String::new();
        let mut terminated = false;

        while !self.scanner.is_eof() {
            if self.scanner.check_len(2).is_ok() && self.scanner.peek_len(2)? == "*/" {
                raw += &self.scanner.read_len(2)?;
                terminated = true;

                break;
            }

            let s = &self.scanner.read()?.to_string();

            raw += s;
            text += s;
        }

        Ok(Token::new(
            TokenKind::Comment {
                kind: CommentKind::Block { terminated },
                text,
            },
            raw,
            pos,
        ))
    }

    fn read_kind(&mut self, kind: TokenKind) -> Result<Token> {
        let pos = self.scanner.get_pos();
        let raw = self.scanner.read()?.to_string();

        Ok(Token::new(kind, raw, pos))
    }

    fn read_keyword(&mut self) -> Result<Token> {
        let pos = self.scanner.get_pos();
        let mut raw = self.scanner.read()?.to_string();

        while !self.scanner.is_eof() {
            let ch = self.scanner.peek()?;

            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    raw += &ch.to_string();

                    self.scanner.skip()?;
                }
                _ => break,
            }
        }

        let kind = match raw.as_str() {
            "null" => TokenKind::Lit(LitKind::Null),
            "true" | "false" => TokenKind::Lit(LitKind::Bool),
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "fn" => TokenKind::Fn,
            "use" => TokenKind::Use,
            "from" => TokenKind::From,
            "enum" => TokenKind::Enum,
            "in" => TokenKind::In,
            "as" => TokenKind::As,
            "mod" => TokenKind::Mod,
            "class" => TokenKind::Class,
            "extends" => TokenKind::Extends,
            "nan" => TokenKind::Lit(LitKind::Num),
            "infinity" => TokenKind::Lit(LitKind::Num),
            "while" => TokenKind::While,
            "pub" => TokenKind::Pub,
            "async" => TokenKind::Async,
            "loop" => TokenKind::Loop,
            "for" => TokenKind::For,
            "yield" => TokenKind::Yield,
            "return" => TokenKind::Return,
            "continue" => TokenKind::Continue,
            "break" => TokenKind::Break,
            _ => TokenKind::Ident(IdentKind::Ident),
        };

        Ok(Token::new(kind, raw, pos))
    }

    fn read_raw_ident(&mut self) -> Result<Token> {
        let pos = self.scanner.get_pos();
        let mut raw = self.scanner.read_len(2)?;

        match self.scanner.peek().unwrap_or('0') {
            'a'..='z' | 'A'..='Z' | '_' => raw += &self.scanner.read()?.to_string(),
            _ => {
                return Ok(Token::new(
                    TokenKind::Ident(IdentKind::Raw { terminated: false }),
                    raw,
                    pos,
                ))
            }
        }

        while !self.scanner.is_eof() {
            let ch = self.scanner.peek()?;

            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    raw += &ch.to_string();

                    self.scanner.skip()?;
                }
                _ => break,
            }
        }

        Ok(Token::new(
            TokenKind::Ident(IdentKind::Raw { terminated: true }),
            raw,
            pos,
        ))
    }

    fn read_text(
        &mut self,
        create_kind: fn(value: String, terminated: bool) -> TokenKind,
        sign: &str,
    ) -> Result<Token> {
        let sign_len = sign.len();
        let pos = self.scanner.get_pos();
        let mut raw = self.scanner.read_len(sign_len)?;
        let mut raw_without_sign = String::new();
        let mut terminated = false;

        while !self.scanner.is_eof() {
            if self.scanner.check_len(sign_len).is_ok() && self.scanner.peek_len(sign_len)? == sign
            {
                raw += &self.scanner.read_len(sign_len)?;
                terminated = true;

                break;
            }

            let ch = self.scanner.read()?;
            let s = &ch.to_string();

            raw += s;
            raw_without_sign += s;

            if !self.scanner.is_eof() && ch == '\\' {
                let ch = self.scanner.read()?;
                let s = &ch.to_string();

                raw += s;
                raw_without_sign += s;
            }
        }

        Ok(Token::new(
            create_kind(raw_without_sign, terminated),
            raw,
            pos,
        ))
    }

    fn read_string(&mut self) -> Result<Token> {
        self.read_text(
            |raw_without_sign, terminated| {
                TokenKind::Lit(LitKind::Str {
                    terminated,
                    raw_without_sign,
                })
            },
            "\"",
        )
    }

    fn read_char(&mut self) -> Result<Token> {
        self.read_text(
            |raw_without_sign, terminated| {
                TokenKind::Lit(LitKind::Char {
                    terminated,
                    raw_without_sign,
                })
            },
            "'",
        )
    }

    pub fn read(&mut self) -> Result<Token> {
        if self.scanner.is_eof() {
            return Ok(Token::new(TokenKind::Eof, "", self.scanner.get_pos()));
        }

        let token = match self.scanner.peek()? {
            '+' => self.read_by(vec![
                ("+", TokenKind::BinOp(BinOpKind::Plus)),
                ("+=", TokenKind::BinOpEq(BinOpKind::Plus)),
            ]),
            '-' => self.read_by(vec![
                ("-", TokenKind::BinOp(BinOpKind::Minus)),
                ("-=", TokenKind::BinOpEq(BinOpKind::Minus)),
            ]),
            '*' => self.read_by(vec![
                ("*", TokenKind::BinOp(BinOpKind::Star)),
                ("*=", TokenKind::BinOpEq(BinOpKind::Star)),
            ]),
            '/' => {
                if self.scanner.check_len(2).is_ok() {
                    let sign = self.scanner.peek_len(2)?;

                    match sign.as_str() {
                        "//" => return self.read_line_comment(),
                        "/*" => return self.read_block_comment(),
                        _ => {}
                    };
                }

                self.read_by(vec![
                    ("/", TokenKind::BinOp(BinOpKind::Slash)),
                    ("/=", TokenKind::BinOpEq(BinOpKind::Slash)),
                ])
            }
            '%' => self.read_by(vec![
                ("%", TokenKind::BinOp(BinOpKind::Percent)),
                ("%=", TokenKind::BinOpEq(BinOpKind::Percent)),
            ]),
            '^' => self.read_by(vec![
                ("^", TokenKind::BinOp(BinOpKind::Caret)),
                ("^=", TokenKind::BinOpEq(BinOpKind::Caret)),
            ]),
            '&' => self.read_by(vec![
                ("&", TokenKind::BinOp(BinOpKind::And)),
                ("&=", TokenKind::BinOpEq(BinOpKind::And)),
                ("&&", TokenKind::BinOp(BinOpKind::AndAnd)),
                ("&&=", TokenKind::BinOpEq(BinOpKind::AndAnd)),
            ]),
            '|' => self.read_by(vec![
                ("|", TokenKind::BinOp(BinOpKind::Or)),
                ("|=", TokenKind::BinOpEq(BinOpKind::Or)),
                ("||", TokenKind::BinOp(BinOpKind::OrOr)),
                ("||=", TokenKind::BinOpEq(BinOpKind::OrOr)),
            ]),
            '<' => self.read_by(vec![
                ("<", TokenKind::Lt),
                ("<=", TokenKind::Le),
                ("<<", TokenKind::BinOp(BinOpKind::ShiftL)),
                ("<<=", TokenKind::BinOpEq(BinOpKind::ShiftL)),
            ]),
            '>' => self.read_by(vec![
                (">", TokenKind::Gt),
                (">=", TokenKind::Ge),
                (">>", TokenKind::BinOp(BinOpKind::ShiftR)),
                (">>=", TokenKind::BinOpEq(BinOpKind::ShiftR)),
            ]),
            '!' => self.read_by(vec![("!", TokenKind::Not), ("!=", TokenKind::Ne)]),
            '=' => self.read_by(vec![("=", TokenKind::Eq), ("==", TokenKind::EqEq)]),
            '@' => self.read_kind(TokenKind::At),
            '.' => self.read_by(vec![
                (".", TokenKind::Dot),
                ("..", TokenKind::DotDot),
                ("..=", TokenKind::DotDotEq),
            ]),
            ',' => self.read_kind(TokenKind::Comma),
            ';' => self.read_kind(TokenKind::Semi),
            '(' => self.read_kind(TokenKind::LParen),
            ')' => self.read_kind(TokenKind::RParen),
            '{' => self.read_kind(TokenKind::LBrace),
            '}' => self.read_kind(TokenKind::RBrace),
            '[' => self.read_kind(TokenKind::LBracket),
            ']' => self.read_kind(TokenKind::RBracket),
            '\'' => self.read_char(),
            '"' => self.read_string(),
            'a'..='z' | 'A'..='Z' | '_' => {
                if self.scanner.check_len(2).is_ok() && self.scanner.peek_len(2)? == "r#" {
                    self.read_raw_ident()
                } else {
                    self.read_keyword()
                }
            }
            ch => {
                if ch.is_whitespace() {
                    self.read_whitespace()
                } else {
                    Err(Error::InvalidCharacter {
                        chr: ch,
                        pos: self.scanner.get_pos(),
                    })
                }
            }
        }?;

        Ok(token)
    }
}
