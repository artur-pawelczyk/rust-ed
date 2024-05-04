use std::{error::Error, fmt::Display, io::Write};

pub struct Editor {
    pub buffer: Buffer,
    pub mode: EditorMode,
    pub line: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            mode: Default::default(),
            line: 1
        }
    }
}

#[derive(Default)]
pub struct Buffer {
    pub contents: String,
}

#[derive(Default, PartialEq)]
pub enum EditorMode {
    #[default] Command, Insert, Quit
}

pub struct CommandContext<'a> {
    pub line: usize,
    pub output: &'a mut dyn Write,
}

impl<'a> CommandContext<'a> {
    pub fn with_output<W: Write>(output: &'a mut W) -> Self {
        Self {
            output,
            line: 1
        }
    }

    pub fn line(self, line: usize) -> Self {
        Self {
            line,
            ..self
        }
    }
}

pub trait EditorFn {
    fn apply(&self, ed: &mut Editor, ctx: &mut CommandContext) -> Result<(), CommandError>;
}

impl<F> EditorFn for F
where F: Fn(&mut Editor, &mut CommandContext) -> Result<(), CommandError>
{
    fn apply(&self, ed: &mut Editor, ctx: &mut CommandContext) -> Result<(), CommandError> {
        self(ed, ctx)
    }
}

#[derive(Debug)]
pub enum CommandError {
    IOError(std::io::Error),
    Generic,
}

impl Error for CommandError {
}

impl From<std::io::Error> for CommandError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "command error")
    }
}
