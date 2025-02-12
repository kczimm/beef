use crate::config::Config;

#[derive(Debug, Default)]
pub struct History(Vec<String>);

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_command(&mut self, command: &str) {
        self.0.push(command.to_string());
    }

    pub fn save(&mut self) {}

    pub fn load(&mut self, config: &Config) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_load() {
        let mut tmpfile = tempfile::tempfile().unwrap();
    }
}
