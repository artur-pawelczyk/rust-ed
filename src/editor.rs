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

pub trait EditorFn<T> {
    fn apply(&self, ed: &mut Editor, cmd: &CommandEnum) -> Result<(), CommandError>;
}

impl<F> EditorFn<&CommandEnum> for F
where F: Fn(&mut Editor, &CommandEnum) -> Result<(), CommandError>
{
    fn apply(&self, ed: &mut Editor, cmd: &CommandEnum) -> Result<(), CommandError> {
        self(ed, cmd)
    }
}

impl<F> EditorFn<()> for F
where F: Fn(&mut Editor) -> Result<(), CommandError>
{
    fn apply(&self, ed: &mut Editor, _: &CommandEnum) -> Result<(), CommandError> {
        self(ed)
    }
}

impl<F> EditorFn<usize> for F
where F: Fn(&mut Editor, usize) -> Result<(), CommandError>
{
    fn apply(&self, ed: &mut Editor, cmd: &CommandEnum) -> Result<(), CommandError> {
        match cmd {
            CommandEnum::Line(n) => self(ed, *n),
            _ => Err(CommandError)
        }
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
