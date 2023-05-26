use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Scanner {
    chars: Vec<char>,
    len: usize,
    pos: usize,
}

impl Scanner {
    pub fn new<T>(input: T) -> Self
    where
        T: Into<String>,
    {
        let input: String = input.into();

        Self {
            chars: input.chars().collect(),
            len: input.len(),
            pos: 0,
        }
    }

    pub fn check_pos(&self, pos: usize) -> Result<()> {
        if pos < self.len {
            Ok(())
        } else {
            Err(Error::OutOfRange)
        }
    }

    pub fn check_len(&self, len: usize) -> Result<()> {
        self.check_pos(self.pos + len - 1)
    }

    pub fn is_eot(&self) -> bool {
        self.check_pos(self.pos).is_err()
    }

    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn read_string(&mut self, len: usize) -> Result<String> {
        let s = self.peek_string(len)?;

        self.skip_chars(len)?;

        Ok(s)
    }

    pub fn peek_string(&self, len: usize) -> Result<String> {
        self.peek_string_with_offset(0, len)
    }

    pub fn peek_string_with_offset(&self, offset: usize, len: usize) -> Result<String> {
        let start = self.pos + offset;
        let end = start + len;

        self.check_pos(end - 1)?;

        Ok(self
            .chars
            .get(start..end)
            .ok_or(Error::OutOfRange)?
            .iter()
            .collect())
    }

    pub fn peek_char_offset(&self, offset: usize) -> Result<char> {
        let pos = self.pos + offset;

        self.check_pos(pos)?;

        self.chars.get(pos).ok_or(Error::OutOfRange).copied()
    }

    pub fn peek_char(&self) -> Result<char> {
        self.peek_char_offset(0)
    }

    pub fn skip_chars(&mut self, len: usize) -> Result<()> {
        self.check_len(len)?;

        self.pos += len;

        Ok(())
    }

    pub fn skip_char(&mut self) -> Result<()> {
        self.skip_chars(1)
    }

    pub fn read_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;

        self.skip_char()?;

        Ok(ch)
    }
}
