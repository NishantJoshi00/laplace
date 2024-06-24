use crate::types::{TExpression, TFunction, Token};

pub fn evaluate_function(function: TFunction, input: TExpression) -> Vec<TExpression> {
    let token = function.variable.1;
    function
        .expression
        .into_iter()
        .map(|expression| apply_input(expression, token.clone(), input.clone()))
        .collect()
}

pub fn apply_input(expr: TExpression, token: Token, input: TExpression) -> TExpression {
    match expr {
        TExpression::Function(func) => TExpression::Function(TFunction {
            variable: func.variable,
            expression: func
                .expression
                .into_iter()
                .map(|exp| apply_input(exp, token.clone(), input.clone()))
                .collect(),
        }),
        TExpression::Variable(expr) => {
            if expr.1 == token {
                input
            } else {
                TExpression::Variable(expr)
            }
        }
    }
}

pub fn evaluate_expression(expr: TExpression, input: TExpression) -> Vec<TExpression> {
    match expr {
        TExpression::Function(func) => func
            .expression
            .into_iter()
            .map(|exp| apply_input(exp, func.variable.1.clone(), input.clone()))
            .collect(),
        TExpression::Variable(_) => vec![expr],
    }
}
