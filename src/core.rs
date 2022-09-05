use std::str::Chars;

/// Represents the state of the typing process.
#[derive(Debug, Clone)]
pub struct TypingState {
    /// The lines of text to be typed.
    lines: Vec<String>,

    /// The index of the current line.
    pub line_idx: usize,

    /// The index of the current character.
    pub char_idx: usize,

    /// A string that collects the last
    /// wrongly typed characters.
    pub wrongs: String,
}

/// Represents some state change
/// happened after some method call
/// on TypingState.
#[derive(Debug, PartialEq)]
pub enum StateChange {
    /// Current line was changed.
    Line,

    /// Current character was changed.
    Char,

    /// Wrong character was added or removed.
    Wrong,

    /// No state change happened.
    None,
}

impl TypingState {
    pub fn new(lines: Vec<String>, line_idx: usize) -> Self {
        TypingState {
            lines,
            line_idx,
            char_idx: 0,
            wrongs: String::default(),
        }
    }

    pub fn get_lines(&self) -> &[String] {
        &self.lines
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn type_char(&mut self, new_char: char) -> StateChange {
        let to_type = self.current_line_chars().skip(self.char_idx).next();
        if self.wrongs.is_empty() {
            match to_type {
                Some(target) if target == new_char => {
                    // Correct
                    self.char_idx += 1;
                    return StateChange::Char;
                }
                // End of file
                None if self.line_idx == self.lines.len() - 1 => return StateChange::None,
                _ => {}
            }
        }
        // Wrong
        self.wrongs.push(new_char);
        return StateChange::Wrong;
    }

    pub fn line_break(&mut self) -> StateChange {
        if self.lines.len() <= self.line_idx {
            // ignore line break at last line
            return StateChange::None;
        }
        if self.wrongs.is_empty() && self.current_line_chars().count() <= self.char_idx {
            // correct line break
            self.char_idx = 0;
            self.line_idx += 1;
            return StateChange::Line;
        }
        // wrong line break
        self.wrongs.push('\n');
        return StateChange::Wrong;
    }

    pub fn backspace(&mut self) -> StateChange {
        if !self.wrongs.is_empty() {
            // erasing previous typos
            self.wrongs.pop();
            return StateChange::Wrong;
        }
        if self.char_idx > 0 {
            // go back a char
            self.char_idx -= 1;
            return StateChange::Char;
        }
        if self.line_idx > 0 {
            // go back a line and put cursor at the end
            self.line_idx -= 1;
            self.char_idx = self.current_line_chars().count();
            return StateChange::Line;
        }
        // nothing to erase
        return StateChange::None;
    }

    pub fn current_line_chars(&self) -> Chars {
        match self.lines.get(self.line_idx) {
            Some(line) => line.chars(),
            None => unreachable!(),
        }
    }
}
