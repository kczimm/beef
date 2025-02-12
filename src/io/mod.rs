pub struct IOHandler;

impl IOHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn print_output(&self, output: &str) {
        print!("{output}")
    }

    pub fn print_error(&self, err: &str) {
        eprint!("{err}")
    }
}
