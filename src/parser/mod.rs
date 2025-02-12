mod expansion;
mod syntax;
mod tokenizer;

#[derive(Debug)]
pub struct ParsedCommand {
    pub command: String,
    pub args: Vec<String>,
}

impl ParsedCommand {
    /// Checks if this command is an exit command.
    pub fn is_exit_command(&self) -> bool {
        self.command == "exit"
    }
}

pub fn parse(input: &str) -> Result<ParsedCommand, String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                if !in_quotes && !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            ' ' if !in_quotes && !current.is_empty() => {
                parts.push(current.clone());
                current.clear();
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    if parts.is_empty() {
        return Err("no command provided".to_string());
    }

    let args = parts[1..].iter().map(|s| s.trim().to_string()).collect();

    Ok(ParsedCommand {
        command: parts[0].trim().to_string(),
        args,
    })
}
