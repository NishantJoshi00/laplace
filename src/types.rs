//!
//! First Principles:
//! - Everything is a function or a value
//! - There can only exist 2 things
//!     - `a` where `a` is a function or a value
//!     - `a(b)` where both `a` and `b` are function or a value

use rand::Rng;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Function {
    pub variable: Variable,
    pub expression: Vec<Expression>,
}

impl Function {
    pub fn from_tuple((variable, expression): (Variable, Vec<Expression>)) -> Self {
        Self {
            variable,
            expression,
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Variable(pub char);

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Expression {
    Function(Function),
    Variable(Variable),
    AliasLink(AliasLink),
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Alias {
    pub variable: AliasLink,
    pub expression: Expression,
}

impl Alias {
    pub fn from_tuple((variable, expression): (AliasLink, Expression)) -> Self {
        Self {
            variable,
            expression,
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct AliasLink(pub String);

// -----------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Token {
    Bound(String),
    Free,
}

impl Token {
    pub fn new_bound() -> Self {
        const CHARSET: &[u8] = b"01234567890abcdef";
        const LENGTH: usize = 16;
        let mut rng = rand::thread_rng();
        let token: String = (0..LENGTH)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        Self::Bound(token)
    }
}

#[derive(Clone, Debug)]
pub struct TFunction {
    pub variable: TVariable,
    pub expression: Vec<TExpression>,
}

#[derive(Clone, Debug)]
pub struct TVariable(pub char, pub Token);

#[derive(Clone, Debug)]
pub enum TExpression {
    Function(TFunction),
    Variable(TVariable),
}
