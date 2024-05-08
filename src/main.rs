mod editor;
mod commands;
mod map;

use std::{error::Error, fmt::Display, fs::File, io::{Read, Write}};

use editor::{Buffer, Editor, EditorMode};
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

    let mut editor = Editor { buffer, mode: EditorMode::Command, line: 1 };

    let mut cmd_map = CommandMap::default();
    cmd_map.bind("a", "append", cmds::append);
    cmd_map.bind("l", "list", cmds::list);
    cmd_map.bind("p", "print-line", cmds::print_line);
    cmd_map.bind("q", "quit", cmds::quit);
    cmd_map.bind_number("goto-line", cmds::goto_line);

    while editor.mode != EditorMode::Quit {
        if let Err(e) = run_cycle(&mut editor, &cmd_map) {
            println!("{:?}", e);
        }
    }

    Ok(())
}

fn run_cycle(editor: &mut Editor, cmd_map: &CommandMap) -> Result<(), Box<dyn Error>> {
    match editor.mode {
        EditorMode::Command => {
            let cmd_str = read_command()?;
            let cmd = cmd_map.lookup(&cmd_str).ok_or(CommandParseError::CommandNotFound)?;

            cmd.run(editor)?;
            Ok(())
        },
        EditorMode::Quit => { return Ok(()); },
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

#[derive(Debug)]
enum CommandParseError {
    CommandNotFound,
}

impl Error for CommandParseError {
}

impl Display for CommandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandNotFound => write!(f, "No such command"),
        }
    }
}
