use std::fs::File;
use std::io::Read;

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use pest::Parser as PestParser;

use crate::args::Args;
// use crate::parse::Parser;
use crate::parse::{LmcParser, Rule};

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
    let mut file = File::open(args.file.clone()).context(format!("opening '{}'", args.file))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .context(format!("reading {}", args.file))?;
    // let res
    println!(
        "{:#?}",
        LmcParser::parse(Rule::program, &buf).context("while parsing")?
    );
    // let mut parser = Parser::new(Box::new(
    // File::open(args.file.clone()).context(format!("opening '{}'", args.file))?,
    // ));
    // println!(
    // "{:?}",
    // parser.parse().context(format!("parsing '{}'", args.file))
    // );
    Ok(())
}
