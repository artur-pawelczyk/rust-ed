use crate::editor::{CommandContext, CommandError, Editor, EditorMode};

pub fn list(ed: &mut Editor, ctx: &mut CommandContext) -> Result<(), CommandError> {
    writeln!(ctx.output, "{}", ed.buffer.contents)?;
    Ok(())
}

pub fn print_line(ed: &mut Editor, ctx: &mut CommandContext) -> Result<(), CommandError> {
    if let Some(line) = ed.buffer.contents.lines().nth(ed.line - 1) {
        writeln!(ctx.output, "{}", line)?;
        Ok(())
    } else {
        Err(CommandError::Generic)
    }
}

pub fn append(ed: &mut Editor, _: &mut CommandContext) -> Result<(), CommandError> {
    ed.mode = EditorMode::Insert;
    Ok(())
}

pub fn goto_line(ed: &mut Editor, ctx: &mut CommandContext) -> Result<(), CommandError> {
    ed.line = ctx.destination.shift(ed.line);
    Ok(())
}

pub fn quit(ed: &mut Editor, _: &mut CommandContext) -> Result<(), CommandError> {
    ed.mode = EditorMode::Quit;
    Ok(())
}

pub fn noop(_: &mut Editor, _: &mut CommandContext) -> Result<(), CommandError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use super::*;

    #[test]
    fn test_list() {
        let mut ed = Editor::default();
        let mut buf = BufWriter::new(Vec::new());
        let mut ctx = CommandContext::with_output(&mut buf);

        ed.buffer.contents.push_str("the content");
        list(&mut ed, &mut ctx).unwrap();

        let output = buf.into_inner().unwrap();
        assert_eq!(output, b"the content\n");
    }

    #[test]
    fn test_print_line() {
        let mut ed = Editor::default();
        let mut buf = BufWriter::new(Vec::new());
        let mut ctx = CommandContext::with_output(&mut buf);

        ed.buffer.contents.push_str("first line\nsecond line");
        ed.line = 2;
        print_line(&mut ed, &mut ctx).unwrap();

        let output = buf.into_inner().unwrap();
        assert_eq!(output, b"second line\n");
    }

    #[test]
    fn test_goto_line() {
        let mut ed = Editor::default();
        let mut out = std::io::stdout();

        let mut ctx = CommandContext::with_output(&mut out).line(100);
        goto_line(&mut ed, &mut ctx).unwrap();
        assert_eq!(ed.line, 100);

        let mut ctx = CommandContext::with_output(&mut out).line_relative(5);
        goto_line(&mut ed, &mut ctx).unwrap();
        assert_eq!(ed.line, 105);

        let mut ctx = CommandContext::with_output(&mut out).line_relative(-200);
        goto_line(&mut ed, &mut ctx).unwrap();
        assert_eq!(ed.line, 1);
    }
}
