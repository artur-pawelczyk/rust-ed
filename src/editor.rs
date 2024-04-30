use std::{error::Error, fmt::Display};

pub struct Editor {
    pub buffer: Buffer,
    pub mode: EditorMode,
}

pub struct Buffer {
    pub contents: String,
}

#[derive(PartialEq)]
pub enum EditorMode {
    Command, Insert, Quit
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CommandEnum {
    Append,
    List,
    Quit,
    Line(usize),
    Noop,
}

pub struct CommandContext;

impl CommandContext {
    pub fn line(&self) -> usize {
        1
    }
}

pub trait EditorFn {
    fn apply(&self, ed: &mut Editor, ctx: &CommandContext) -> Result<(), CommandError>;
}

impl<F> EditorFn for F
where F: Fn(&mut Editor, &CommandContext) -> Result<(), CommandError>
{
    fn apply(&self, ed: &mut Editor, ctx: &CommandContext) -> Result<(), CommandError> {
        self(ed, ctx)
    }
}

#[derive(Debug)]
pub struct CommandError;

impl Error for CommandError {
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "command error")
    }
}
