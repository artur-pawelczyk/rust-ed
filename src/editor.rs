use std::{error::Error, fmt::Display};

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

pub struct CommandContext {
    pub line: usize
}

impl Default for CommandContext {
    fn default() -> Self {
        Self {
            line: 1
        }
    }
}

impl CommandContext {
    pub fn line(self, line: usize) -> Self {
        Self {
            line,
            ..self
        }
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
