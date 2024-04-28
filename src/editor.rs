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

pub trait EditorFn {
    fn apply(&self, ed: &mut Editor, cmd: &Command) -> Result<(), CommandError>;
}

pub struct CommandError;
