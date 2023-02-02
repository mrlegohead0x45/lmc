use crate::aliases::*;

#[derive(Debug)]
pub(crate) enum Location {
    Label(String),
    Address(Address),
}

#[derive(Debug)]
pub(crate) enum Instruction {
    Halt,
    Add(Location),
    Subtract(Location),
    Store(Location),
    Load(Location),
    BranchAlways(Location),
    BranchIfZero(Location),
    BranchIfPositive(Location),
    Input,
    Output,
    Data(String, Option<Value>),
    Label(String),
}
