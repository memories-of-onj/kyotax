use std::ops::Range;

use itertools::Itertools;

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Scanner {
    chars: Vec<char>,
    len: usize,
    pos: usize,
}

impl From<&str> for Scanner {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Scanner {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for Scanner {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

impl Scanner {
    pub fn new<T>(input: T) -> Self
    where
        T: Into<String>,
    {
        let input: String = input.into();
        let chars = input.chars().collect_vec();
        let len = chars.len();

        Self { len, chars, pos: 0 }
    }

    fn check(&self, pos: usize) -> Result<()> {
        if pos < self.len {
            Ok(())
        } else {
            Err(Error::OutOfRange)
        }
    }

    pub fn check_len(&self, len: usize) -> Result<()> {
        self.check(self.pos + len - 1)
    }

    pub fn is_eof(&self) -> bool {
        self.check(self.pos).is_err()
    }

    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn read_len(&mut self, len: usize) -> Result<String> {
        let s = self.peek_len(len)?;

        self.skip_len(len)?;

        Ok(s)
    }

    pub fn peek_len(&self, len: usize) -> Result<String> {
        self.peek_range(0..len)
    }

    pub fn peek_range(&self, i: Range<usize>) -> Result<String> {
        let start = self.pos + i.start;
        let end = start + i.end;

        self.check(end - 1)?;

        Ok(self
            .chars
            .get(start..end)
            .ok_or(Error::OutOfRange)?
            .iter()
            .collect())
    }

    pub fn peek(&self) -> Result<char> {
        self.check(self.pos)?;

        self.chars.get(self.pos).ok_or(Error::OutOfRange).copied()
    }

    pub fn skip_len(&mut self, len: usize) -> Result<()> {
        self.check_len(len)?;

        self.pos += len;

        Ok(())
    }

    pub fn skip(&mut self) -> Result<()> {
        self.skip_len(1)
    }

    pub fn read(&mut self) -> Result<char> {
        let ch = self.peek()?;

        self.skip()?;

        Ok(ch)
    }
}
