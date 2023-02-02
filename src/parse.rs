use std::io::Read;

use crate::instruction::Instruction;
use anyhow::{Context, Result};

pub(crate) struct Parser {
    file: Box<dyn Read>,
    // current_char: Option<char>,
    // position: Position,
}

impl Parser {
    pub(crate) fn new(file: Box<dyn Read>) -> Self {
        Self { file }
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<Instruction>> {
        let mut result = Vec::new();

        let mut code = String::new();
        self.file.read_to_string(&mut code)?;
        // self.consume_whitespace()?;
        // loop {
        //     let c = self.advance()?;
        //     if let ';' | '#' | '/' = c {
        //         if c == '/' {}
        //     }
        // }
        for (line_number, line) in code.split('\n').enumerate() {
            if let Some(inst) = self
                .parse_line(line)
                .context(format!("parsing line {}", line_number + 1))?
            {
                result.push(inst);
            }
            // result.push(self.parse_line(line)?);
        }
        Ok(result)
    }

    fn parse_line(&self, line: &str) -> Result<Option<Instruction>> {
        let mut inst = None;
        let mut chars = line.chars();
        while let Some(char) = chars.next() {
            if let ' ' | '\t' | '\r' = char {
                continue;
            }
            if let ';' | '#' | '/' = char {
                // if let k
            }
        }
        Ok(inst)
    }

    // fn advance(&mut self) -> Result<char> {
    //     let mut buf: [u8; 1] = [0x20];
    //     self.file
    //         .read_exact(&mut buf)
    //         .context(format!("Trying to open {}", self.position.filename()))?;
    //     self.position.advance(buf[0] as char);
    //     Ok(buf[0] as char)
    // }

    // fn consume_whitespace(&mut self) -> Result<()> {
    //     while let ' ' | '\t' | '\n' | '\r' = self.advance()? {}

    //     Ok(())
    // }
}
