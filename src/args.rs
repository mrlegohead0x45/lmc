use clap::Parser;
#[derive(Parser, Debug)]
pub(crate) struct Args {
    /// The file to run
    pub(crate) file: String,
}
