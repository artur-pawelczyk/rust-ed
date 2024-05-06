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
    pub destination: LineOffset,
    pub output: &'a mut dyn Write,
}

impl<'a> CommandContext<'a> {
    pub fn with_output<W: Write>(output: &'a mut W) -> Self {
        Self {
            output,
            destination: LineOffset::default(),
        }
    }

    pub fn line(self, line: usize) -> Self {
        Self {
            destination: LineOffset::Absolute(line),
            ..self
        }
    }

    pub fn line_relative(self, n: isize) -> Self {
        Self {
            destination: LineOffset::Relative(n),
            ..self
        }
    }
}

pub enum LineOffset {
    Relative(isize),
    Absolute(usize),
}

impl Default for LineOffset {
    fn default() -> Self {
        Self::Relative(0)
    }
}

impl LineOffset {
    pub fn shift(&self, l: usize) -> usize {
        match self {
            Self::Absolute(i) => *i,
            Self::Relative(i) => {
                let x = l as isize + i;
                x.try_into().unwrap_or(1)
            }
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
