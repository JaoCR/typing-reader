use stable_eyre::eyre::{eyre, Result};
use std::{cmp::min, vec};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
};

use crate::core::{StateChange, TypingState};
use crate::file;

pub struct State {
    pub typing: TypingState,
    pub filename: String,
    pub text_height: u16,
}

pub fn init(filename: &str, text_height: u16) -> Result<State> {
    State::new(filename, text_height)
}

impl State {
    pub fn new(filename: &str, text_height: u16) -> Result<Self> {
        let lines = file::load(filename)?;
        let text_state = match lines
            .get(0)
            .ok_or(eyre!("File is empty."))?
            .split("typing-reader:")
            .nth(1)
        {
            Some(header) => {
                let curr_line = header
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0);
                let lines = lines.into_iter().skip(1).collect();
                TypingState::new(lines, curr_line)
            }
            None => TypingState::new(lines, 0),
        };

        Ok(Self {
            typing: text_state,
            filename: filename.to_string(),
            text_height,
        })
    }

    pub fn type_char(&mut self, new_char: char) {
        self.typing.type_char(new_char);
    }

    pub fn line_break(&mut self) {
        if self.typing.line_break() == StateChange::Line {
            self.save().unwrap();
        }
    }

    pub fn backspace(&mut self) {
        if self.typing.backspace() == StateChange::Line {
            self.save().unwrap();
        }
    }

    pub fn save(&self) -> Result<()> {
        file::save(
            &self.filename,
            self.typing.get_lines(),
            self.typing.line_idx,
        )
    }

    pub fn current_text(&self) -> Text {
        let mut text = Text::default();
        text.extend(self.lines_before_current());
        text.extend(self.current_typing_lines());
        text.extend(self.lines_after_current());
        text
    }

    fn lines_before_current(&self) -> impl Iterator<Item = Spans> {
        let to_include = min(self.typing.line_idx, 2);
        let to_skip = self.typing.line_idx - to_include;
        self.typing
            .get_lines()
            .iter()
            .skip(to_skip)
            .take(to_include)
            .map(|line| {
                Spans::from(Span::styled(
                    line,
                    Style::default().add_modifier(Modifier::DIM),
                ))
            })
    }

    fn lines_after_current(&self) -> impl Iterator<Item = Spans> {
        self.typing
            .get_lines()
            .iter()
            .skip(self.typing.line_idx + 1)
            .take(self.text_height as usize)
            .map(|line| Spans::from(Span::raw(line)))
    }

    /// Split current line into 4 String parts:
    /// 1. text before cursor
    /// 2. character at cursor, can be a line break symbol if at end of the line
    /// 3. text after cursor up until end of the line
    /// 4. extra line break symbol if cursor is not at the end of the line
    fn split_current_line(&self) -> (String, String, String, String) {
        let mut line = self.typing.current_line_chars();
        let before = String::from_iter(line.by_ref().take(self.typing.char_idx));
        let remaining = String::from_iter(line.by_ref());
        let cursor: String;
        let after: String;
        let ending: String;
        if self.typing.wrongs.is_empty() {
            let mut not_typed_chars = remaining.chars();
            cursor = not_typed_chars.next().unwrap_or('⏎').to_string();
            after = String::from_iter(not_typed_chars);
            ending = if remaining.is_empty() {
                " ".to_string()
            } else {
                "⏎".to_string()
            };
        } else {
            cursor = "⌫".to_string();
            after = remaining;
            ending = "⏎".to_string();
        }
        (before, cursor, after, ending)
    }

    /// Current typing lines as vector of tui::text::Spans objects.
    ///
    /// These include the beginning and end of the line the cursor is
    /// at the original text, as well as all wrong characters typed
    /// in the middle of it, including line breaks, that are represented
    /// by separating each line in a different Spans object.
    fn current_typing_lines(&self) -> Vec<Spans> {
        let sty = Style::default();
        let sty_dim = sty.add_modifier(Modifier::DIM);
        let sty_wrong = sty.bg(Color::Red);
        let sty_dim_wrong = sty_wrong.add_modifier(Modifier::DIM);
        let sty_cursor = sty.bg(Color::DarkGray);

        let (before, cursor, after, ending) = self.split_current_line();

        if self.typing.wrongs.contains('\n') {
            let mut wrong_lines = self.typing.wrongs.split('\n');
            // take first line
            let first = Spans::from(vec![
                Span::raw(before),
                Span::styled(wrong_lines.next().unwrap(), sty_wrong),
                Span::styled(ending.to_owned(), sty_dim_wrong),
            ]);
            // take last line
            let last = Spans::from(vec![
                Span::styled(wrong_lines.next_back().unwrap(), sty_wrong),
                Span::styled(cursor, sty_cursor),
                Span::styled(after, sty),
                Span::styled(ending.to_owned(), sty_dim),
            ]);
            // join with middle
            let mut lines = vec![first];
            lines.extend(wrong_lines.map(|line| {
                Spans::from(vec![
                    Span::styled(line, sty_wrong),
                    Span::styled(ending.to_owned(), sty_dim_wrong),
                ])
            }));
            lines.push(last);
            lines
        } else {
            vec![Spans::from(vec![
                Span::styled(before, sty_dim),
                Span::styled(&self.typing.wrongs, sty_wrong),
                Span::styled(cursor, sty_cursor),
                Span::styled(after, sty),
                Span::styled(ending, sty_dim),
            ])]
        }
    }
}
