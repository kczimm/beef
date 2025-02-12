mod expansion;
mod syntax;
mod tokenizer;

pub fn parse(input: &str) -> Result<String, String> {
    Ok(input.strip_suffix('\n').unwrap_or(input).to_string())
}
