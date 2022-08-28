use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use stable_eyre::eyre::{bail, Context, Error, Result};
use std::{
    fs::File,
    io::{self, BufRead, Stdout, Write},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod core;
mod tui_app;

#[derive(Parser, Debug)]
#[clap(author, version, about, setting(clap::AppSettings::ColoredHelp))]
struct Args {
    #[clap(value_parser, forbid_empty_values = true)]
    filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(&args.filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let mut terminal = setup_terminal()?;

    match tui_app::run(&mut terminal, &mut lines, &args.filename) {
        Ok(_) => {
            cleanup_terminal(terminal)?;
            Ok(())
        }
        Err(e) => {
            cleanup_terminal(terminal)?;
            bail!(e)
        }
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn cleanup_terminal<B>(mut terminal: Terminal<B>) -> Result<()>
where
    B: Backend,
    B: Write,
{
    {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok::<(), Error>(())
    }
    .wrap_err("FAILED TO RESTORE TERMINAL")?;
    Ok(())
}
