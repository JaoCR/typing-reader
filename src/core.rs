use std::{
    collections::VecDeque,
    str::Chars,
    time::{Duration, Instant},
};

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

    /// Queue of the last character/second measures
    /// recorded from each of the last typed lines.
    durations: CPSQueue,

    /// The time when the current line was started.
    last_line_start: Option<Instant>,
}

#[derive(Debug, Clone)]
struct CPSQueue {
    inner_deque: VecDeque<f32>,
    max_size: usize,
}

impl CPSQueue {
    fn new(max_size: usize) -> Self {
        Self {
            inner_deque: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    fn push(&mut self, duration: Duration, chars: usize) {
        self.inner_deque
            .push_back(chars as f32 / duration.as_secs_f32());
        if self.inner_deque.len() > self.max_size {
            self.inner_deque.pop_front();
        }
    }

    fn average(&self) -> Option<f32> {
        if self.inner_deque.is_empty() {
            None
        } else {
            Some(self.inner_deque.iter().sum::<f32>() / self.inner_deque.len() as f32)
        }
    }

    //fn resize(&mut self, new_size: usize) {
    //    self.max_size = new_size;
    //    while self.inner_deque.len() > self.max_size {
    //        self.inner_deque.pop_front();
    //    }
    //}
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
            durations: CPSQueue::new(5),
            last_line_start: None,
        }
    }

    pub fn moving_avg_cps(&self) -> Option<f32> {
        self.durations.average()
    }

    fn start_timing(&mut self) {
        if let None = self.last_line_start {
            self.last_line_start = Some(Instant::now());
        }
    }

    fn cancel_timing(&mut self) {
        self.last_line_start = None;
    }

    fn finish_timing(&mut self) {
        if let Some(start) = self.last_line_start {
            let duration = start.elapsed();
            self.durations.push(duration, self.char_idx);
            self.last_line_start = None;
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
                None if self.line_idx == self.lines.len() - 1 => {
                    self.finish_timing();
                    return StateChange::None;
                }
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
            self.finish_timing();
            return StateChange::None;
        }
        if self.wrongs.is_empty() && self.current_line_chars().count() <= self.char_idx {
            // correct line break
            self.finish_timing();
            self.start_timing();
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
            self.cancel_timing();
            self.line_idx -= 1;
            self.char_idx = self.current_line_chars().count();
            return StateChange::Line;
        }
        // nothing to erase
        self.cancel_timing();
        return StateChange::None;
    }

    pub fn current_line_chars(&self) -> Chars {
        match self.lines.get(self.line_idx) {
            Some(line) => line.chars(),
            None => unreachable!(),
        }
    }
}
