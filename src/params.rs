use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, setting(clap::AppSettings::ColoredHelp))]
pub struct Args {
    #[clap(value_parser, forbid_empty_values = true)]
    pub filename: String,
}

pub fn load() -> Args {
    Args::parse()
}
