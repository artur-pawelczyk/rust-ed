mod editor;
mod commands;
mod map;

use std::{error::Error, fmt::Display, fs::File, io::{Read, Write}};

use editor::{Buffer, CommandEnum, Editor, EditorMode};
use commands as cmds;
use map::CommandMap;

fn main() -> Result<(), Box<dyn Error>> {
    let buffer = if let Some(path) = std::env::args().skip(1).next() {
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;
        Buffer { contents }
    } else {
        Buffer { contents: String::new() }
    };

    let mut editor = Editor { buffer, mode: EditorMode::Command };

    let mut cmd_map = CommandMap::default();
    cmd_map.bind("a", "append", cmds::append);
    cmd_map.bind("l", "list", cmds::list);
    cmd_map.bind("q", "quit", cmds::quit);
    cmd_map.bind_number("goto-line", cmds::goto_line);

    loop {
        if editor.mode == EditorMode::Command {
            let cmd = read_command().and_then(|cmd| {
                Ok(cmd_map.lookup(&cmd).ok_or_else(|| Box::new(CommandParseError)))
            })??;

            cmd.run(&mut editor)?;
        } else if editor.mode == EditorMode::Insert {
            let new_content = read_content()?;
            editor.buffer.contents.push_str(&new_content);
            editor.mode = EditorMode::Command;
        }
    }
}

fn read_command() -> Result<String, Box<dyn Error>> {
    let mut out = std::io::stdout();
    write!(out, "> ")?;
    out.flush()?;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn read_content() -> Result<String, Box<dyn Error>> {
    let mut buf = String::new();
    let mut last: usize = 0;
    loop {
        let chars_read = std::io::stdin().read_line(&mut buf)?;
        if buf[last..last+chars_read].trim_end() == "." {
            buf.truncate(last);
            return Ok(buf);
        }

        last += chars_read;
    }
}

impl CommandEnum {
    fn parse(s: &str) -> Result<Self, CommandParseError> {
        let cmd = s.split_ascii_whitespace().next().unwrap_or("");
        if cmd.is_empty() {
            return Ok(Self::Noop);
        }

        if let Ok(line_number) = cmd.parse::<usize>() {
            Ok(Self::Line(line_number))
        } else {
            match cmd {
                "l" => Ok(CommandEnum::List),
                "a" => Ok(CommandEnum::Append),
                "q" => Ok(CommandEnum::Quit),
                _ => Err(CommandParseError),
            }
        }
    }        
}

#[derive(Debug)]
struct CommandParseError;

impl Error for CommandParseError {
}

impl Display for CommandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "command parse error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command() -> Result<(), Box<dyn Error>> {
        let s = "l";
        let command = CommandEnum::parse(&s)?;
        assert_eq!(command, CommandEnum::List);

        let s = "a";
        let command = CommandEnum::parse(&s)?;
        assert_eq!(command, CommandEnum::Append);

        let s = "12";
        let command = CommandEnum::parse(&s)?;
        assert_eq!(command, CommandEnum::Line(12));

        Ok(())
    }
}
