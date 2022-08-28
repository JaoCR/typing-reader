use stable_eyre::eyre::{bail, eyre, Result};
use std::{
    io::{BufRead, Lines},
    str::Chars,
};

pub struct State {
    pub lines: Vec<String>,
    pub line_idx: usize,
    pub char_idx: usize,
    pub wrongs: String,
}

impl State {
    pub fn new<R: BufRead>(lines: &mut Lines<R>, line_idx: usize) -> Result<Self> {
        let mut lines_vec = vec![];
        for line in lines.by_ref() {
            if let Ok(l) = line {
                lines_vec.push(l.trim_end().to_string());
            } else {
                bail!("Error when parsing line")
            }
        }
        Ok(State {
            // Using Result::from_iter turns the iterator of Results in a
            // Result that might be a Ok(with an iterator), it also stops
            // evaluating on the first error. Useful because Lines yields
            // Results and we need to map it to it's trimmed slices.
            lines: lines_vec,
            line_idx,
            char_idx: 0,
            wrongs: String::default(),
        })
    }

    pub fn type_char(&mut self, c: char) -> Result<()> {
        let current_line: Vec<char> = self.current_line_chars()?.collect();
        // the followind matches! checks if the current target character
        // equals the typed character.
        if self.wrongs.is_empty() && matches!(current_line.get(self.char_idx), Some(t) if *t == c) {
            // character typed correctly
            self.char_idx += 1;
        } else {
            // character typed incorrectly
            self.wrongs.push(c);
        }
        Ok(())
    }

    pub fn line_break(&mut self) -> Result<()> {
        if self.wrongs.is_empty() && self.current_line_chars()?.count() <= self.char_idx {
            // correct line break
            self.char_idx = 0;
            self.line_idx += 1;
        } else {
            // wrong line break
            self.wrongs.push('\n')
        }
        Ok(())
    }

    pub fn backspace(&mut self) -> Result<()> {
        if self.wrongs.is_empty() {
            if self.char_idx <= 0 {
                if self.line_idx > 0 {
                    // go back a line
                    self.line_idx -= 1;
                    self.char_idx = self.current_line_chars()?.count();
                } // else do nothing
            } else {
                // go back a char
                self.char_idx -= 1;
            }
        } else {
            // erasing wrong stuff
            self.wrongs.pop();
        }
        Ok(())
    }

    pub fn current_line_chars(&self) -> Result<Chars> {
        Ok(self
            .lines
            .get(self.line_idx)
            .ok_or(eyre!("No line at this index."))?
            .chars())
    }
}
