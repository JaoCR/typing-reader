/// Manages the terminal.
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use stable_eyre::eyre::{Result, WrapErr};
use std::{
    io::{self, Stdout},
    ops::{Deref, DerefMut},
};
use tui::{backend::CrosstermBackend, Terminal};

pub struct Term(Terminal<CrosstermBackend<Stdout>>);

/// Sacures that the terminal will be cleaned up properly.
impl Drop for Term {
    fn drop(&mut self) {
        cleanup(self).unwrap_or_else(|e| eprintln!("Error cleaning up terminal: {}", e));
    }
}

/// To treat every Term as a reference to the underlying Terminal.
impl Deref for Term {
    type Target = Terminal<CrosstermBackend<Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// To treat every Term as a mutable reference to the underlying Terminal.
impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn setup() -> Result<Term> {
    enable_raw_mode().wrap_err("Error enabling raw mode.")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).wrap_err("Error entering alternate screen.")?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Term(
        Terminal::new(backend).wrap_err("Error setting up Terminal object.")?,
    ))
}

fn cleanup(term: &mut Term) -> Result<()> {
    disable_raw_mode().wrap_err("Error disabling raw mode.")?;
    execute!(term.backend_mut(), LeaveAlternateScreen)
        .wrap_err("Error leaving alternate screen.")?;
    term.show_cursor().wrap_err("Error showing cursor.")?;
    Ok(())
}
