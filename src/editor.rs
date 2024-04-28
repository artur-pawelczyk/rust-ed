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
    Command, Insert
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Append,
    List,
    Quit,
    Line(usize),
    Noop,
}

pub trait EditorFn<T> {
    fn apply(&self, ed: &mut Editor, cmd: &Command) -> Result<(), CommandError>;
}

impl<F> EditorFn<&Command> for F
where F: Fn(&mut Editor, &Command) -> Result<(), CommandError>
{
    fn apply(&self, ed: &mut Editor, cmd: &Command) -> Result<(), CommandError> {
        self(ed, cmd)
    }
}

impl<F> EditorFn<()> for F
where F: Fn(&mut Editor) -> Result<(), CommandError>
{
    fn apply(&self, ed: &mut Editor, _: &Command) -> Result<(), CommandError> {
        self(ed)
    }
}

impl<F> EditorFn<usize> for F
where F: Fn(&Editor, usize) -> Result<(), CommandError>
{
    fn apply(&self, ed: &mut Editor, cmd: &Command) -> Result<(), CommandError> {
        match cmd {
            Command::Line(n) => self(ed, *n),
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
