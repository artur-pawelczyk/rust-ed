use crate::editor::{CommandEnum, CommandError, Editor, EditorMode};

pub fn list(ed: &mut Editor, _: &CommandEnum) -> Result<(), CommandError> {
    println!("{}", ed.buffer.contents);
    Ok(())
}

pub fn append(ed: &mut Editor) -> Result<(), CommandError> {
    ed.mode = EditorMode::Insert;
    Ok(())
}

pub fn goto_line(_: &mut Editor, line: usize) -> Result<(), CommandError> {
    println!("go to line {line}");
    Ok(())
}

pub fn quit(ed: &mut Editor) -> Result<(), CommandError> {
    ed.mode = EditorMode::Quit;
    Ok(())
}

pub fn noop(_: &mut Editor) -> Result<(), CommandError> {
    Ok(())
}
