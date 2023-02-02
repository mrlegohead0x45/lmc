use std::fs::File;

use anyhow::{Context, Result};
use clap::Parser as ClapParser;

use crate::args::Args;
use crate::parse::Parser;

mod args;
mod computer;
mod instruction;
mod parse;
// mod position;

mod aliases {
    /// We have 256 bytes of memory
    pub type Address = u8;

    /// Each address holds two bytes
    pub type Value = u16;

    /// The size of our memory
    pub const MEMORY_LENGTH: usize = 2_usize.pow(Address::BITS);
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut parser = Parser::new(Box::new(
        File::open(args.file.clone()).context(format!("opening '{}'", args.file))?,
    ));
    println!(
        "{:?}",
        parser.parse().context(format!("parsing '{}'", args.file))
    );
    Ok(())
}
