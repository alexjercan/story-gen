use super::string;
use crate::error::Error;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::eof,
    error::ParseError, multi::many0, sequence::delimited, AsChar, Compare, IResult, InputLength,
    InputTake, InputTakeAtPosition, Parser,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Comment(String),
    Say(String, String),
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Action::Comment(comment) => write!(f, "// {}", comment),
            Action::Say(name, text) => write!(f, "{}: {}", name, text),
        }
    }
}

fn lexeme<I, O, F, E>(mut f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition,
    F: Parser<I, O, E>,
    E: ParseError<I>,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    move |input: I| {
        let (input, output) = f.parse(input)?;
        let (input, _) = multispace0(input)?;
        Ok((input, output))
    }
}

fn symbol<T, I, E>(s: T) -> impl FnMut(I) -> IResult<I, I, E>
where
    T: InputLength + Clone,
    I: InputTake + Compare<T> + InputTakeAtPosition,
    E: ParseError<I>,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    lexeme(tag(s))
}

fn comment(input: &str) -> IResult<&str, Action> {
    let (input, (_, comment)) = symbol("comment")
        .and(delimited(
            symbol("("),
            lexeme(string::parse_string),
            symbol(")"),
        ))
        .parse(input)?;
    Ok((input, Action::Comment(comment.to_string())))
}

fn say(input: &str) -> IResult<&str, Action> {
    let (input, (_, ((name, _), text))) = symbol("say")
        .and(delimited(
            symbol("("),
            lexeme(
                string::parse_string
                    .and(symbol(","))
                    .and(lexeme(string::parse_string)),
            ),
            symbol(")"),
        ))
        .parse(input)?;
    Ok((input, Action::Say(name.to_string(), text.to_string())))
}

fn action(input: &str) -> IResult<&str, Action> {
    alt((comment, say)).parse(input)
}

fn actions_(input: &str) -> IResult<&str, Vec<Action>> {
    delimited(multispace0, many0(action), eof).parse(input)
}

pub fn actions(input: &str) -> Result<Vec<Action>, Error> {
    let (_, actions) = actions_(input).map_err(|err| Error::ParserError(err.to_string()))?;
    Ok(actions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment() {
        assert_eq!(
            comment("comment (\"hello world\")"),
            Ok(("", Action::Comment("hello world".to_string())))
        );
    }

    #[test]
    fn test_say() {
        assert_eq!(
            say("say (\"hello world\", \"hello world\")"),
            Ok((
                "",
                Action::Say("hello world".to_string(), "hello world".to_string())
            ))
        );
    }

    #[test]
    fn test_actions() {
        assert_eq!(
            actions(
                "comment (\"hello world\")\n\
                 say (\"hello world\", \"hello world\")"
            ),
            Ok(vec![
                Action::Comment("hello world".to_string()),
                Action::Say("hello world".to_string(), "hello world".to_string())
            ])
        );
    }
}
