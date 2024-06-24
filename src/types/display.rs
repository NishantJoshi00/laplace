use color_eyre::owo_colors::OwoColorize;
use std::fmt::Display;

use super::*;

impl Display for TVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            Token::Bound(_) => write!(f, "{}", self.0.green()),
            Token::Free => write!(f, "{}", self.0.red()),
        }
    }
}

impl Display for TFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "λ{}", self.variable.green())?;
        write!(f, ".")?;
        write!(
            f,
            "({})",
            self.expression
                .iter()
                .map(|exp| exp.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Display for TExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TExpression::Function(func) => write!(f, "{}", func),
            TExpression::Variable(var) => write!(f, "{}", var),
        }
    }
}

impl TExpression {
    pub fn display_expressions(exprs: Vec<TExpression>) -> String {
        format!(
            "({})",
            exprs
                .iter()
                .map(|exp| exp.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
