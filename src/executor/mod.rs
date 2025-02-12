use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{environment::Environment, job_control::JobControl, parser::ParsedCommand};

mod pipeline;
mod process;
mod redirection;

pub struct Executor {
    _environment: Environment,
    _job_control: JobControl,
}

impl Executor {
    pub fn new(environment: Environment, job_control: JobControl) -> Self {
        Self {
            _environment: environment,
            _job_control: job_control,
        }
    }

    pub fn execute(&self, parsed_command: &ParsedCommand) -> Result<String, String> {
        Ok(parsed_command.command.clone())
    }
}

fn find_files<P: AsRef<Path>>(start_path: P, pattern: &str) -> Vec<PathBuf> {
    let mut found = Vec::new();

    // Helper function to recursively walk through directories
    fn walk_dir(dir: &Path, found: &mut Vec<PathBuf>, pattern: &str) {
        if dir.is_dir() {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        walk_dir(&path, found, pattern);
                    } else if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                        if file_name.contains(pattern) {
                            found.push(path);
                        }
                    }
                }
            }
        }
    }

    walk_dir(start_path.as_ref(), &mut found, pattern);
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_files() {
        let files = find_files("/bin", "ls");
        assert_eq!(files, vec![PathBuf::from("/bin/ls")]);
    }
}
