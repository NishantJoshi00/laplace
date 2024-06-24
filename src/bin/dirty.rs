use color_eyre::Result;
use laplace::types::TExpression;

fn main() -> Result<()> {
    color_eyre::install()?;

    let file = std::fs::read_to_string("samples/lexer/dirty.l")?;

    let ast = laplace::lexer::parse(&file).and_then(laplace::token::tokenize_ast)?;

    let i_main = ast.get("MAIN").expect("No MAIN function found").clone();
    let i_expr = ast.get("APPLY").expect("No APPLY found").clone();

    println!("{}", i_main);

    let i_main = laplace::eval::evaluate_expression(i_main, i_expr);

    println!("{}", TExpression::display_expressions(i_main));

    Ok(())
}
