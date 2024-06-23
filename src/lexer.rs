//!
//! This module consists of all the lexical structures and parsers required to parse the lambda
//! expression.
//!
//! The lambda expression is a simple language with only 3 tokens:
//! 1. `λ` - lambda
//! 2. `.` - dot
//! 3. 'a-zA-Z' - variable
//! 4. '()' - parenthesis
//!
//!
//!
//!
//!
//!
//! consider everything as `b` or `a(b)` i.e. an expression
//! where `a` is a function `\x.expression`
//! and `b` is either a function or a value
//!
//!

use nom::bytes::complete as bcomplete;
use nom::character::complete as ccomplete;
use nom::error::{convert_error, VerboseError};
use nom::{branch, combinator, error, multi, sequence, AsChar, IResult, Parser};

use crate::types::{self, Alias, AliasLink, Function, Variable};

fn parse_space_safe<'a, F, O, E>(parser: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: error::ParseError<&'a str>,
{
    sequence::delimited(ccomplete::multispace0, parser, ccomplete::multispace0)
}

fn parse_variable<'a, E>(input: &'a str) -> IResult<&'a str, Variable, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str>,
{
    let ctx = "variable";
    let (rest, this) = error::context(ctx, ccomplete::anychar).parse(input)?;

    match this {
        i if i.is_alpha() && i.is_lowercase() => Ok((rest, Variable(i))),
        _ => error::context(ctx, combinator::fail).parse(input),
    }
}

fn parse_function<'a, E>(input: &'a str) -> IResult<&'a str, Function, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str>,
{
    let ctx = "function";

    error::context(
        ctx,
        sequence::preceded(
            ccomplete::one_of("λ\\"),
            sequence::separated_pair(parse_variable, bcomplete::tag("."), parse_expressions),
        )
        .map(Function::from_tuple),
    )
    .parse(input)
}

fn parse_alias_link<'a, E>(input: &'a str) -> IResult<&'a str, AliasLink, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str>,
{
    let ctx = "alias_link";

    error::context(
        ctx,
        bcomplete::take_while1(|data: char| data.is_alpha() && data.is_uppercase())
            .map(|data: &'a str| AliasLink(data.to_string())),
    )
    .parse(input)
}

fn parse_expression<'a, E>(input: &'a str) -> IResult<&'a str, types::Expression, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str>,
{
    let ctx = "expression";

    error::context(
        ctx,
        branch::alt((
            sequence::delimited(
                bcomplete::tag("("),
                branch::alt((
                    parse_function.map(types::Expression::Function),
                    parse_alias_link.map(types::Expression::AliasLink),
                    parse_variable.map(types::Expression::Variable),
                )),
                bcomplete::tag(")"),
            ),
            branch::alt((
                parse_function.map(types::Expression::Function),
                parse_alias_link.map(types::Expression::AliasLink),
                parse_variable.map(types::Expression::Variable),
            )),
        )),
    )
    .parse(input)
}

fn parse_expressions<'a, E>(input: &'a str) -> IResult<&'a str, Vec<types::Expression>, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str>,
{
    let ctx = "expressions";

    error::context(
        ctx,
        branch::alt((
            sequence::delimited(
                bcomplete::tag("("),
                multi::many1(parse_space_safe(branch::alt((
                    sequence::delimited(
                        bcomplete::tag("("),
                        parse_space_safe(parse_expression),
                        bcomplete::tag(")"),
                    ),
                    parse_expression,
                )))),
                bcomplete::tag(")"),
            ),
            multi::many1(branch::alt((
                sequence::delimited(
                    bcomplete::tag("("),
                    parse_space_safe(parse_expression),
                    bcomplete::tag(")"),
                ),
                parse_expression,
            ))),
        )),
    )
    .parse(input)
}

fn parse_alias<'a, E>(input: &'a str) -> IResult<&'a str, Alias, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str>,
{
    let ctx = "alias";

    error::context(
        ctx,
        parse_space_safe(sequence::preceded(
            bcomplete::tag("let "),
            sequence::separated_pair(
                parse_space_safe(parse_alias_link),
                bcomplete::tag("="),
                parse_space_safe(parse_expression),
            ),
        ))
        .map(types::Alias::from_tuple),
    )
    .parse(input)
}

fn parse_file<'a, E>(input: &'a str) -> IResult<&'a str, Vec<Alias>, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str>,
{
    let ctx = "file";

    error::context(ctx, parse_space_safe(multi::many0(parse_alias))).parse(input)
}

pub fn parse(input: &str) -> color_eyre::Result<Vec<Alias>> {
    let result = combinator::all_consuming(parse_file::<VerboseError<&str>>).parse(input);
    match result {
        Ok(("", result)) => Ok(result),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            Err(color_eyre::eyre::eyre!("{}", convert_error(input, e)))
        }
        _ => Err(color_eyre::eyre::eyre!("The input wasn't fully consumed.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! f {
        ($variable:expr => $($expr:expr),+) => {
            types::Expression::Function(
            types::Function {
                variable: Variable($variable),
                expression: vec![$($expr,)+],
            })
        };
        (?$variable:expr => $($expr:expr),+) => {
            types::Function {
                variable: Variable($variable),
                expression: vec![$($expr,)+],
            }
        };
    }

    macro_rules! v {
        ($variable:expr) => {
            types::Expression::Variable(Variable($variable))
        };
        (?$variable:expr) => {
            Variable($variable)
        };
    }

    macro_rules! a {
        ($alias:expr) => {
            types::Expression::AliasLink(AliasLink($alias.to_string()))
        };
        (?$alias:expr) => {
            AliasLink($alias.to_string())
        };
    }

    #[test]
    fn test_parse_variable_pass() {
        let data = "ab";
        let expected = v!(?'a');
        let (rest, result) = parse_variable::<()>(data).unwrap();
        assert_eq!(result, expected);
        assert_eq!(rest, "b");
    }

    #[test]
    fn test_parse_variable_fail_1() {
        let data = " b";
        let result = parse_variable::<()>(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_variable_fail_2() {
        let data = "\\b";
        let result = parse_variable::<()>(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_variable_fail_3() {
        let data = "1b";
        let result = parse_variable::<()>(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lambda_pass_1() {
        let data = "λa.b";
        let expected = f!(?'a' => v!('b'));
        let (rest, result) = parse_function::<()>(data).unwrap();
        assert_eq!(result, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_lambda_pass_2() {
        let data = "\\a.b";
        let expected = f!(?'a' => v!('b'));
        let (rest, result) = parse_function::<()>(data).unwrap();
        assert_eq!(result, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_lambda_pass_3() {
        let data = "λa.(b)";
        let expected = f!(?'a' => v!('b'));
        let (rest, result) = parse_function::<()>(data).unwrap();
        assert_eq!(result, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_lambda_pass_4() {
        let data = "λa.bc";
        let expected = f!(?'a' => v!('b'), v!('c'));
        let (rest, result) = parse_function::<()>(data).unwrap();
        assert_eq!(result, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_lambda_pass_5() {
        let data = "λa.(bc)";

        let expected = f!(?'a' => v!('b'), v!('c'));

        let (rest, result) = parse_function::<()>(data).unwrap();
        assert_eq!(result, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_lambda_pass_6() {
        let data = "λa.b(c)";
        let expected = f!(?'a' => v!('b'), v!('c'));
        let (rest, result) = parse_function::<()>(data).unwrap();
        assert_eq!(result, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_lambda_pass_7() {
        let data = "λa.(b(c))";
        let expected = f!(?'a' => v!('b'), v!('c'));
        let (rest, result) = parse_function::<()>(data).unwrap();
        assert_eq!(result, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_lambda_pass_8() {
        let data = "λa.(b)c";
        let expected = f!(?'a' => v!('b'));
        let (rest, result) = parse_function::<()>(data).unwrap();
        assert_eq!(result, expected);
        assert_eq!(rest, "c");
    }

    #[test]
    fn test_parse_lambda_fail_1() {
        let data = "a";
        let result = parse_function::<()>(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lambda_fail_2() {
        let data = "λa.";
        let result = parse_function::<()>(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lambda_fail_3() {
        let data = "λa.()b";
        let result = parse_function::<()>(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_alias_link_pass_1() {
        let data = "\\a.A";
        let expected = f!(?'a' => a!("A"));
        let (rest, result) = parse_function::<()>(data).unwrap();
        assert_eq!(result, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_file_pass_1() {
        let file = include_str!("../samples/lexer/script_pass.l");

        let expected = vec![
            Alias::from_tuple((a!(?"MAIN"), f!('x' => v!('x')))),
            Alias::from_tuple((a!(?"TRUE"), f!('x' => f!('y' => v!('x'))))),
        ];

        let (_rest, result) = parse_file::<()>(file).unwrap();

        assert_eq!(result, expected);
    }
}
