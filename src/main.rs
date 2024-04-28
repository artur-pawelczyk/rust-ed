mod editor;

use std::{error::Error, fmt::Display, fs::File, io::{Read, Write}};

use editor::{Buffer, Editor, EditorMode};

fn main() -> Result<(), Box<dyn Error>> {
    let buffer = if let Some(path) = std::env::args().skip(1).next() {
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;
        Buffer { contents }
    } else {
        Buffer { contents: String::new() }
    };

    let mut editor = Editor { buffer, mode: EditorMode::Command };

    loop {
        if editor.mode == EditorMode::Command {
            match read_command()? {
                Command::List => println!("{}", editor.buffer.contents),
                Command::Append => editor.mode = EditorMode::Insert,
                Command::Line(_) => todo!(),
                Command::Quit => return Ok(()),
                Command::Noop => {},
            }
        } else if editor.mode == EditorMode::Insert {
            let new_content = read_content()?;
            editor.buffer.contents.push_str(&new_content);
            editor.mode = EditorMode::Command;
        }
    }
}

fn read_command() -> Result<Command, Box<dyn Error>> {
    let mut out = std::io::stdout();
    write!(out, "> ")?;
    out.flush()?;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Command::parse(&buf).map_err(|err| Box::new(err) as Box<dyn Error>)
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

#[derive(Debug, PartialEq)]
enum Command {
    Append,
    List,
    Quit,
    Line(usize),
    Noop,
}

impl Command {
    fn parse(s: &str) -> Result<Self, CommandParseError> {
        let cmd = s.split_ascii_whitespace().next().unwrap_or("");
        if cmd.is_empty() {
            return Ok(Self::Noop);
        }

        if let Ok(line_number) = cmd.parse::<usize>() {
            Ok(Self::Line(line_number))
        } else {
            match cmd {
                "l" => Ok(Command::List),
                "a" => Ok(Command::Append),
                "q" => Ok(Command::Quit),
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
        let command = Command::parse(&s)?;
        assert_eq!(command, Command::List);

        let s = "a";
        let command = Command::parse(&s)?;
        assert_eq!(command, Command::Append);

        let s = "12";
        let command = Command::parse(&s)?;
        assert_eq!(command, Command::Line(12));

        Ok(())
    }
}
