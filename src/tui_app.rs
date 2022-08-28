use crossterm::event::{self, Event, KeyCode};
use stable_eyre::eyre::{bail, eyre, Result};
use std::io::{BufRead, Lines};
use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Borders, Wrap},
    Terminal,
};

use crate::core::State;

impl State {
    fn current_text(&self) -> Result<Text> {
        let mut text = self.current_line_text()?;
        text.extend(match self.lines.get(self.line_idx + 1..) {
            Some(lines) => lines
                .iter()
                .map(|line| Spans::from(line.to_owned()))
                .collect(),
            None => vec![],
        });
        Ok(text)
    }
    fn current_line_text(&self) -> Result<Text> {
        let mut line = self.current_line_chars()?;
        let before = String::from_iter(line.by_ref().take(self.char_idx));
        let not_typed = String::from_iter(line.by_ref());
        let (cursor_char, after, ending) = if self.wrongs.is_empty() {
            let mut not_typed_chars = not_typed.chars();
            (
                not_typed_chars.next().unwrap_or('⏎'),
                String::from_iter(not_typed_chars),
                if not_typed.is_empty() { ' ' } else { '⏎' },
            )
        } else {
            ('⌫', not_typed, '⏎')
        };
        // split the wrongs on '\n' keeping emptys
        let mut inner_lines = vec![vec![Span::styled(
            before,
            Style::default().add_modifier(Modifier::DIM),
        )]];
        let mut wrong_line = vec![];
        for c in self.wrongs.chars() {
            if c == '\n' {
                // using unwrap because vector already starts with content inside.
                let last = inner_lines.last_mut().unwrap();
                last.push(Span::styled(
                    String::from_iter(&wrong_line),
                    Style::default().bg(Color::Red),
                ));
                last.push(Span::styled(
                    "⏎",
                    Style::default().add_modifier(Modifier::DIM).bg(Color::Red),
                ));
                inner_lines.push(vec![]);
                wrong_line.clear();
            } else {
                wrong_line.push(c);
            }
        }
        let last = inner_lines.last_mut().unwrap();
        last.push(Span::styled(
            String::from_iter(wrong_line),
            Style::default().bg(Color::Red),
        ));
        last.push(Span::styled(
            cursor_char.to_string(),
            Style::default().bg(Color::DarkGray),
        ));
        last.push(Span::raw(after));
        last.push(Span::styled(
            ending.to_string(),
            Style::default().add_modifier(Modifier::DIM),
        ));
        let spans: Vec<Spans> = inner_lines.iter().map(|v| Spans(v.to_owned())).collect();
        Ok(Text::from(spans))
    }
}

pub fn run<'a, B: Backend, R: BufRead>(
    terminal: &mut Terminal<B>,
    lines: &mut Lines<R>,
    title: &str,
) -> Result<()> {
    // initialize state
    let mut state = {
        let header = lines.next().ok_or(eyre!("No file contents."))??;
        let start = header
            .find("typing-reader")
            .ok_or(eyre!("No typing-reader header."))?;
        let argv: Vec<&str> = header[start..].split_whitespace().collect();
        if argv.len() < 2 {
            bail!("Header missing current line.")
        };
        let current_line: usize = argv[1].parse()?;
        State::new(lines, current_line)
    }?;

    loop {
        let current_text = state.current_text()?;

        // draw terminal
        terminal.draw(|frame| {
            let size = frame.size();

            let block = tui::widgets::Block::default()
                .title(Span::styled(
                    title,
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL);

            let par = tui::widgets::Paragraph::new(current_text)
                .block(block)
                .wrap(Wrap { trim: true });

            frame.render_widget(par, size);
        })?;

        // handle events
        match event::read()? {
            Event::Key(e) => match e.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Backspace => {
                    state.backspace()?;
                }
                KeyCode::Enter => {
                    state.line_break()?;
                }
                KeyCode::Char(c) => {
                    state.type_char(c)?;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
