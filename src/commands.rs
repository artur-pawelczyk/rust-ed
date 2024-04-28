use crate::editor::{Command, CommandError, Editor, EditorMode};

pub fn list(ed: &mut Editor, _: &Command) -> Result<(), CommandError> {
    println!("{}", ed.buffer.contents);
    Ok(())
}

pub fn append(ed: &mut Editor) -> Result<(), CommandError> {
    ed.mode = EditorMode::Insert;
    Ok(())
}
