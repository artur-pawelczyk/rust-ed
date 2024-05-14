use crate::editor::{CommandContext, CommandError, Editor, EditorMode};

pub fn list(ed: &mut Editor, ctx: &mut CommandContext) -> Result<(), CommandError> {
    for (n, line) in ed.buffer.contents.lines().enumerate() {
        writeln!(ctx.output, "{} {}", n+1, line)?;
    }
    Ok(())
}

pub fn print_line(ed: &mut Editor, ctx: &mut CommandContext) -> Result<(), CommandError> {
    if let Some(line) = ed.buffer.contents.lines().nth(ed.buffer.line - 1) {
        writeln!(ctx.output, "{}", line)?;
        Ok(())
    } else {
        Err(CommandError::Generic)
    }
}

pub fn append(ed: &mut Editor, ctx: &mut CommandContext) -> Result<(), CommandError> {
    let text = ctx.input.read().map_err(|_| CommandError::Read)?;
    let target_line = ctx.destination.shift(ed.buffer.line);
    let start = ed.buffer.contents.char_indices()
        .filter(|(_, c)| *c == '\n')
        .map(|(i, _)| i + 1)
        .skip(target_line - 1)
        .next()
        .unwrap_or(0);
    ed.buffer.contents.insert_str(start, &text);
    ed.buffer.line = text.lines().count() + ed.buffer.line;
    Ok(())
}

pub fn change_line(ed: &mut Editor, ctx: &mut CommandContext) -> Result<(), CommandError> {
    let text = ctx.input.read().map_err(|_| CommandError::Read)?;
    let target_line = ctx.destination.shift(ed.buffer.line);
    let mut lines = ed.buffer.contents.char_indices().filter(|(_, c)| *c == '\n').skip(target_line - 2);
    let start = lines.next().map(|(i, _)| i).ok_or(CommandError::Generic)? + 1;
    let end = lines.next().map(|(i, _)| i).unwrap_or(ed.buffer.contents.len());
    ed.buffer.contents.replace_range(start..end, text.trim_end());
    Ok(())
}

pub fn goto_line(ed: &mut Editor, ctx: &mut CommandContext) -> Result<(), CommandError> {
    ed.buffer.line = ctx.destination.shift(ed.buffer.line);
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

    use crate::editor::TextInput;

    use super::*;

    #[test]
    fn test_list() {
        let mut ed = Editor::default();
        let mut buf = BufWriter::new(Vec::new());
        let mut ctx = CommandContext::with_output(&mut buf);

        ed.buffer.contents.push_str("the content");
        list(&mut ed, &mut ctx).unwrap();

        let output = buf.into_inner().unwrap();
        assert_eq!(output, b"1 the content\n");
    }

    #[test]
    fn test_print_line() {
        let mut ed = Editor::default();
        let mut buf = BufWriter::new(Vec::new());
        let mut ctx = CommandContext::with_output(&mut buf);

        ed.buffer.contents.push_str("first line\nsecond line");
        ed.buffer.line = 2;
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
        assert_eq!(ed.buffer.line, 100);

        let mut ctx = CommandContext::with_output(&mut out).line_relative(5);
        goto_line(&mut ed, &mut ctx).unwrap();
        assert_eq!(ed.buffer.line, 105);

        let mut ctx = CommandContext::with_output(&mut out).line_relative(-200);
        goto_line(&mut ed, &mut ctx).unwrap();
        assert_eq!(ed.buffer.line, 1);
    }

    #[test]
    fn test_append() {
        let mut ed = Editor::default();
        let mut out = std::io::sink();
        let mut ctx = CommandContext::with_output(&mut out);

        ed.buffer.contents.push_str("first\n");
        ctx.input = &ConstInput("end\n");
        append(&mut ed, &mut ctx).unwrap();

        assert_eq!(ed.buffer.contents, "first\nend\n");
        assert_eq!(ed.buffer.line, 2);

        ed.buffer.line = 1;
        ctx.input = &ConstInput("middle\n");
        append(&mut ed, &mut ctx).unwrap();

        assert_eq!(ed.buffer.contents, "first\nmiddle\nend\n");
        assert_eq!(ed.buffer.line, 2);
    }

    #[test]
    fn test_change_line() {
        let mut ed = Editor::default();
        let mut out = std::io::sink();
        let mut ctx = CommandContext::with_output(&mut out).line(2);

        ed.buffer.contents.push_str("first\nsecond\nthird\n");
        ctx.input = &ConstInput("changed\n");
        change_line(&mut ed, &mut ctx).unwrap();

        assert_eq!(ed.buffer.contents, "first\nchanged\nthird\n");
    }

    struct ConstInput(&'static str);
    impl TextInput for ConstInput {
        fn read(&self) -> Result<String, ()> {
            Ok(String::from(self.0))
        }
    }
}
