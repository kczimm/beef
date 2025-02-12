use crate::{config::Config, environment::Environment};

pub fn generate_prompt(_environment: &Environment, _config: &Config) -> String {
    format!("$ ")
}
