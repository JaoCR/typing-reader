use stable_eyre::eyre::{Result, WrapErr};

mod core;
mod file;
mod params;
mod state;
mod term;
mod tui;

fn main() -> Result<()> {
    let _params = &params::load();

    let mut _term = term::setup().wrap_err("Error setting up terminal.")?;

    let mut _state = state::init(
        &_params.filename,
        _term.size().and_then(|rect| Ok(rect.height)).unwrap_or(100),
    )
    .wrap_err("Error initializing app's state")?;

    let mut _continue = true;
    while _continue {
        tui::draw(&mut _term, &_state).wrap_err("Error drawing UI.")?;
        _continue =
            tui::process_events(&_term, &mut _state).wrap_err("Error processing events.")?;
        _state.save().wrap_err("Error saving file.")?;
    }
    Ok(())
}
