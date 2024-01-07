use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub(crate) enum Status {
    Success,
    Failure,
}
