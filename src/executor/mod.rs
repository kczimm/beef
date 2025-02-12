use crate::{environment::Environment, job_control::JobControl, parser::ParsedCommand};

mod pipeline;
mod process;
mod redirection;

pub struct Executor;

impl Executor {
    pub fn new(_environment: &Environment, _job_control: &JobControl) -> Self {
        Self
    }

    pub fn execute(&self, parsed_command: &ParsedCommand) -> Result<String, String> {
        Ok(parsed_command.command.clone())
    }
}
