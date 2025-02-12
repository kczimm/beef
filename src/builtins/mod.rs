use std::process;

pub fn execute(command: &str, args: &[String]) -> Result<bool, String> {
    match command.trim() {
        "exit" => exit(args),
        "echo" => echo(args),
        _ => Ok(false),
    }
}

fn exit(args: &[String]) -> Result<bool, String> {
    let exit_code = if !args.is_empty() {
        args[0]
            .parse::<i32>()
            .map_err(|_| String::from("invalid exit code"))?
    } else {
        0
    };

    process::exit(exit_code);
}

fn echo(args: &[String]) -> Result<bool, String> {
    print!("{}", args.join(" "));
    Ok(true)
}
