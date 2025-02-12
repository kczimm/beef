use std::io::{self, Write};

use beef::{config, environment, executor, history, job_control, parser, prompt};

fn main() {
    // Initialize modules
    let mut config = config::Config::new();
    let mut environment = environment::Environment::new();
    let mut history = history::History::new();
    let io_handler = beef::io::IOHandler::new();
    let mut job_control = job_control::JobControl::new();
    let executor = executor::Executor::new(&mut environment, &mut job_control);

    // Load configuration
    config.load();

    // Apply environment settings
    environment.apply_config(&config);

    // Start the REPL loop
    loop {
        // Display prompt
        let prompt = prompt::generate_prompt(&environment, &config);
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        // Read command
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Parse the command
                match parser::parse(&input) {
                    Ok(parsed_command) => {
                        // Check if we should exit the shell
                        if parsed_command == "exit" {
                            break;
                        }

                        // Add to history
                        history.add_command(&input.trim());

                        // Execute the command
                        let execution_result = executor.execute(&parsed_command);

                        // Handle output or errors
                        match execution_result {
                            Ok(output) => io_handler.print_output(&output),
                            Err(e) => io_handler.print_error(&e),
                        }
                    }
                    Err(e) => io_handler.print_error(&e.to_string()),
                }
            }
            Err(e) => {
                io_handler.print_error(&format!("Failed to read line: {}", e));
                break;
            }
        }
    }

    // Save history and configuration before exiting
    history.save();
    config.save();
}
