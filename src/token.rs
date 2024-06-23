//!
//!
//!
//! Evaluation is basically a FUNCTION
//! \A.B(C) -> EXPRESSION
//! where
//!     A: PLACEHOLDER
//!     B: EXPRESSION
//!     C: EXPRESSION
//! EXPRESSION
//! | FUNCTION
//! | VALUE
//!

use crate::types::{Alias, Expression, TExpression, TFunction, TVariable, Token};
use color_eyre::eyre::bail;
use color_eyre::Result;
use std::collections::HashMap;

pub fn tokenize_expression(
    input: Expression,
    registry: &HashMap<String, TExpression>,
    token_registry: HashMap<char, Token>,
) -> Result<TExpression> {
    Ok(match input {
        Expression::Function(fun) => {
            let variable = TVariable(fun.variable.0, Token::new_bound());
            let new_token_registry = {
                let mut cloned_token_registry = token_registry.clone();
                cloned_token_registry.insert(fun.variable.0, variable.1.clone());
                cloned_token_registry
            };

            let expression = fun
                .expression
                .into_iter()
                .map(|exp| tokenize_expression(exp, registry, new_token_registry.clone()))
                .collect::<Result<Vec<TExpression>>>()?;

            TExpression::Function(TFunction {
                variable,
                expression,
            })
        }
        Expression::Variable(var) => {
            if let Some(token) = token_registry.get(&var.0) {
                TExpression::Variable(TVariable(var.0, token.clone()))
            } else {
                TExpression::Variable(TVariable(var.0, Token::Free))
            }
        }
        Expression::AliasLink(alias) => registry
            .get(&alias.0)
            .cloned()
            .ok_or(color_eyre::eyre::eyre!("Alias not found: {}", alias.0))?,
    })
}

pub fn tokenize_alias(input: Alias, registry: &mut HashMap<String, TExpression>) -> Result<()> {
    let name = input.variable.0;
    let token_registry = HashMap::new();
    let expression = tokenize_expression(input.expression, registry, token_registry)?;
    registry.insert(name, expression);
    Ok(())
}

pub fn tokenize_ast(input: Vec<Alias>) -> Result<HashMap<String, TExpression>> {
    let mut registry = HashMap::new();
    input
        .into_iter()
        .try_for_each(|alias| tokenize_alias(alias, &mut registry))?;

    if !registry.contains_key("MAIN") {
        bail!("No MAIN alias found");
    }

    Ok(registry)
}
