use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{environment::Environment, job_control::JobControl, parser::ParsedCommand};

mod pipeline;
mod process;
mod redirection;

pub struct Executor {
    _environment: Environment,
    job_control: JobControl,
}

impl Executor {
    pub fn new(environment: Environment, job_control: JobControl) -> Self {
        Self {
            _environment: environment,
            job_control,
        }
    }

    pub fn execute(&self, parsed_command: ParsedCommand) -> Result<String, String> {
        Ok(
            if let Some(command) = get_command_from_path(&parsed_command.command) {
                let output = Command::new(command)
                    .args(parsed_command.args)
                    .output()
                    .unwrap();
                String::from_utf8(output.stdout).unwrap()
            } else {
                format!("{} not found\n", parsed_command.command)
            },
        )
    }
}

fn get_command_from_path(command: &str) -> Option<PathBuf> {
    for path in env::var("PATH").unwrap_or_default().split(':') {
        if let Some(command) = find_file(path, command) {
            return Some(command);
        }
    }

    None
}

fn find_file<P: AsRef<Path>>(start_path: P, pattern: &str) -> Option<PathBuf> {
    fn walk_dir(dir: &Path, pattern: &str) -> Option<PathBuf> {
        if dir.is_dir() {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        return walk_dir(&path, pattern);
                    } else if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                        if file_name == pattern {
                            return Some(path);
                        }
                    }
                }
            }
        }

        None
    }

    walk_dir(start_path.as_ref(), pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_file() {
        let file = find_file("/bin", "ls");
        assert_eq!(file, Some(PathBuf::from("/bin/ls")));
    }
}
