use crate::editor::{CommandContext, CommandError, Editor, EditorMode};

pub fn list(ed: &mut Editor, _: &CommandContext) -> Result<(), CommandError> {
    println!("{}", ed.buffer.contents);
    Ok(())
}

pub fn append(ed: &mut Editor, _: &CommandContext) -> Result<(), CommandError> {
    ed.mode = EditorMode::Insert;
    Ok(())
}

pub fn goto_line(ed: &mut Editor, ctx: &CommandContext) -> Result<(), CommandError> {
    ed.line = ctx.line();
    Ok(())
}

pub fn quit(ed: &mut Editor, _: &CommandContext) -> Result<(), CommandError> {
    ed.mode = EditorMode::Quit;
    Ok(())
}

pub fn noop(_: &mut Editor, _: &CommandContext) -> Result<(), CommandError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goto_line() {
        let mut ed = Editor::default();
        let ctx = CommandContext(123);

        goto_line(&mut ed, &ctx).unwrap();

        assert_eq!(ed.line, 123);
    }
}
