/// Contains implementations for built-in commands. This module handles internal
/// commands like `cd`, `exit`, `echo`, etc., which do not spawn new processes
/// but are executed within the shell itself.
pub mod builtins;

/// Manages command completion functionality. This pub module provides features
/// for auto-completing commands, file paths, or other shell-specific inputs
/// based on user input.
pub mod completion;

/// Manages configuration settings for the shell. This includes loading, saving,
/// and applying user preferences, themes, or custom settings.
pub mod config;

/// Handles the shell's environment variables. This pub module provides methods to
/// set, get, and manipulate environment variables, which are crucial for
/// process execution and script behavior.
pub mod environment;

/// Responsible for executing commands, either built-in or external. This pub module
/// interacts with the operating system to run commands, manage processes,
/// and handle their outputs or errors.
pub mod executor;

/// Manages command history. This includes storing, retrieving, and searching
/// through previously executed commands for user convenience.
pub mod history;

/// Deals with input/output operations. This pub module manages how data is read
/// from and written to different streams like stdin, stdout, stderr, or files.
pub mod io;

/// Implements job control mechanisms for managing foreground and background
/// jobs, including pausing, resuming, and terminating processes.
pub mod job_control;

/// Parses user input into executable commands. This pub module interprets the
/// shell language, handles syntax, redirection, piping, and command-line arguments.
pub mod parser;

/// Manages the shell's prompt. This includes formatting the prompt string,
/// possibly with dynamic content like current directory, username, or time.
pub mod prompt;

/// Provides support for scripting capabilities within the shell. This could
/// include running scripts, managing script execution context, or providing
/// shell-specific scripting constructs.
mod scripting;
