use clap::ValueEnum;
use clap::builder::Styles;
use clap::builder::styling::AnsiColor;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub(crate) enum Status {
    Success,
    Failure,
}

pub fn get_style() -> clap::builder::Styles {
    let styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default());
    styles
}
