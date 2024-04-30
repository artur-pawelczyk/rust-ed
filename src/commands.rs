use crate::editor::{CommandContext, CommandError, Editor, EditorMode};

pub fn list(ed: &mut Editor, _: &CommandContext) -> Result<(), CommandError> {
    println!("{}", ed.buffer.contents);
    Ok(())
}

pub fn append(ed: &mut Editor, _: &CommandContext) -> Result<(), CommandError> {
    ed.mode = EditorMode::Insert;
    Ok(())
}

pub fn goto_line(_: &mut Editor, ctx: &CommandContext) -> Result<(), CommandError> {
    println!("go to line {}", ctx.line());
    Ok(())
}

pub fn quit(ed: &mut Editor, _: &CommandContext) -> Result<(), CommandError> {
    ed.mode = EditorMode::Quit;
    Ok(())
}

pub fn noop(_: &mut Editor, _: &CommandContext) -> Result<(), CommandError> {
    Ok(())
}
