use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Word(String),
    Number(i64),
    String(String),
    If,
    Else,
    For,
    While,
    Function,
    Let,
    Return,
    Semicolon,
    Equals,
    LBrace,
    RBrace,
    LParent,
    RParent,
    Comma,
    Colon,
    GreaterThan,
    LessThan,
    RightArrow,
    Plus,
}

#[allow(dead_code)]
pub fn tokenize(input: &str) -> IResult<&str, Vec<Token>> {
    many0(preceded(
        multispace0,
        alt((
            map(tag("if"), |_| Token::If),
            map(tag("else"), |_| Token::Else),
            map(tag("for"), |_| Token::For),
            map(tag("while"), |_| Token::While),
            map(tag("fn"), |_| Token::Function),
            map(tag("let"), |_| Token::Let),
            map(tag("return"), |_| Token::Return),
            map(tag(";"), |_| Token::Semicolon),
            map(tag("="), |_| Token::Equals),
            map(tag("{"), |_| Token::LBrace),
            map(tag("}"), |_| Token::RBrace),
            map(tag("("), |_| Token::LParent),
            map(tag(")"), |_| Token::RParent),
            map(tag(","), |_| Token::Comma),
            map(tag(":"), |_| Token::Colon),
            map(tag(">"), |_| Token::GreaterThan),
            map(tag("<"), |_| Token::LessThan),
            map(tag("->"), |_| Token::RightArrow),
            map(tag("+"), |_| Token::Plus),
            map(
                recognize(delimited(tag("\""), take_while1(|c| c != '"'), tag("\""))),
                |s: &str| Token::String(s[1..s.len() - 1].to_string()),
            ),
            map(
                take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                |s: &str| {
                    if let Ok(num) = s.parse::<i64>() {
                        Token::Number(num)
                    } else {
                        Token::Word(s.to_string())
                    }
                },
            ),
        )),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_command() {
        let input = "echo \"Hello, World!\"";
        let (_, tokens) = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Word("echo".to_string()),
                Token::String("Hello, World!".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_variable_declaration() {
        let input = "let name = \"John Doe\";";
        let (_, tokens) = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Word("name".to_string()),
                Token::Equals,
                Token::String("John Doe".to_string()),
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_tokenize_if_statement() {
        let input = "if x > 5 { echo \"x is greater than 5\" }";
        let (_, tokens) = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::Word("x".to_string()),
                Token::GreaterThan,
                Token::Number(5),
                Token::LBrace,
                Token::Word("echo".to_string()),
                Token::String("x is greater than 5".to_string()),
                Token::RBrace,
            ]
        );
    }

    #[test]
    fn test_tokenize_function_declaration() {
        let input = "fn greet(name: String) -> String { return \"Hello, \" + name; }";
        let (_, tokens) = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Function,
                Token::Word("greet".to_string()),
                Token::LParent,
                Token::Word("name".to_string()),
                Token::Colon,
                Token::Word("String".to_string()),
                Token::RParent,
                Token::RightArrow,
                Token::Word("String".to_string()),
                Token::LBrace,
                Token::Return,
                Token::String("Hello, ".to_string()),
                Token::Plus,
                Token::Word("name".to_string()),
                Token::Semicolon,
                Token::RBrace,
            ]
        );
    }

    #[test]
    fn test_tokenize_numbers() {
        let input = "let count = 10;";
        let (_, tokens) = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Word("count".to_string()),
                Token::Equals,
                Token::Number(10),
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_tokenize_multiple_statements() {
        let input = "let x = 5; let y = 10; if x < y { echo \"x is less than y\"; }";
        let (_, tokens) = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Word("x".to_string()),
                Token::Equals,
                Token::Number(5),
                Token::Semicolon,
                Token::Let,
                Token::Word("y".to_string()),
                Token::Equals,
                Token::Number(10),
                Token::Semicolon,
                Token::If,
                Token::Word("x".to_string()),
                Token::LessThan,
                Token::Word("y".to_string()),
                Token::LBrace,
                Token::Word("echo".to_string()),
                Token::String("x is less than y".to_string()),
                Token::Semicolon,
                Token::RBrace,
            ]
        );
    }
}
