use std::{error::Error, fmt::Display, fs::File, io::{Read, Write}};

struct Buffer {
    contents: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = if let Some(path) = std::env::args().skip(1).next() {
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;
        Buffer { contents }
    } else {
        Buffer { contents: String::new() }
    };

    let mut input = String::new();
    while read_command(&mut input).is_ok() {
        match Command::parse(&input)? {
            Command::List => println!("{}", buffer.contents),
            Command::Append(s) => buffer.contents.push_str(s),
        }

        input.clear();
    }

    Ok(())
}

fn read_command(buf: &mut String) -> Result<(), Box<dyn Error>> {
    let mut out = std::io::stdout();
    write!(out, "> ")?;
    out.flush()?;
    std::io::stdin().read_line(buf)?;
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
    Append(&'a str),
    List,
}

impl<'a> Command<'a> {
    fn parse(s: &'a str) -> Result<Self, CommandParseError> {
        match &s[0..1] {
            "l" => Ok(Command::List),
            "a" => {
                let new_content = &s[2..];
                let new_content = new_content.strip_suffix('\n').unwrap_or(new_content);
                Ok(Command::Append(new_content))
            },
            _ => Err(CommandParseError),
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
    fn parse_list_command() -> Result<(), Box<dyn Error>> {
        let s = "l";
        let command = Command::parse(&s)?;
        assert_eq!(command, Command::List);

        Ok(())
    }

    #[test]
    fn parse_append_command() -> Result<(), Box<dyn Error>> {
        let s = "a something";
        let command = Command::parse(&s)?;
        assert_eq!(command, Command::Append("something"));

        let s = "a with newline\n";
        let command = Command::parse(&s)?;
        assert_eq!(command, Command::Append("with newline"));

        Ok(())
    }
}
