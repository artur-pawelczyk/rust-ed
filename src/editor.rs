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
