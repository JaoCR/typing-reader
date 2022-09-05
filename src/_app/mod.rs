use crossterm::event::{self, Event, KeyCode};
use stable_eyre::eyre::{bail, eyre, Context, Result};
use std::{
    fs::File,
    io::{self, BufRead, Lines},
};
use tui::{
    backend::Backend,
    style::{Modifier, Style},
    text::Span,
    widgets::{Borders, Wrap},
    Terminal,
};

use crate::core::TextState;
mod state;
mod term;
mod tui;

pub fn run(filename: &str) -> Result<()> {
    let file = File::open(&filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let mut terminal = term::setup()?;

    match tui::start(&mut terminal, &mut lines, &filename) {
        Ok(_) => {
            term::cleanup(terminal)?;
            Ok(())
        }
        Err(e) => {
            term::cleanup(terminal)?;
            bail!(e)
        }
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
        let current_line_idx: usize = argv[1].parse().wrap_err("Invalid header.")?;
        let lines_vector: Vec<String> = Result::from_iter(lines)?;
        TextState::new(lines_vector, current_line_idx)
    };
}
