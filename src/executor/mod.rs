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
