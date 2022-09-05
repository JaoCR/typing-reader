use crate::{state::State, term::Term};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use stable_eyre::eyre::Result;
use tui::{
    layout::Layout,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Wrap},
};

/// Draw the TUI.
pub fn draw(term: &mut Term, state: &State) -> Result<()> {
    let current_text = state.current_text();
    term.draw(|frame| {
        let size = frame.size();

        let layout = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints(
                [
                    tui::layout::Constraint::Min(1),
                    tui::layout::Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(size);

        let main_block = Block::default().borders(Borders::ALL).title(Span::styled(
            &state.filename,
            Style::default().add_modifier(Modifier::BOLD),
        ));

        let par = tui::widgets::Paragraph::new(current_text)
            .block(main_block)
            .wrap(Wrap { trim: true });
        frame.render_widget(par, layout[0]);

        let footer = tui::widgets::Paragraph::new(format!(
            "Line {}/{}",
            state.typing.line_idx + 1,
            state.typing.line_count()
        ));
        frame.render_widget(footer, layout[1]);
    })?;
    Ok(())
}

const CONTINUE: Result<bool> = Ok(true);
const STOP: Result<bool> = Ok(false);

/// Process events and return whether to continue or not.
pub fn process_events(term: &Term, state: &mut State) -> Result<bool> {
    state.text_height = term.size().and_then(|rect| Ok(rect.height)).unwrap_or(100);
    match event::read()? {
        Event::Key(e) => match e.code {
            KeyCode::Backspace => state.backspace(),
            KeyCode::Enter => state.line_break(),
            KeyCode::Char('q') if e.modifiers == KeyModifiers::CONTROL => return STOP,
            KeyCode::Char(c) => state.type_char(c),
            _ => {}
        },
        _ => {}
    }
    CONTINUE
}
