mod path;
mod variables;

pub struct Environment;

impl Environment {
    pub fn new() -> Self {
        Self
    }

    pub fn apply_config(&self, _config: &crate::config::Config) {}
}
