use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let file = std::fs::read_to_string("samples/lexer/dirty.l")?;

    let ast = laplace::lexer::parse(&file).and_then(laplace::token::tokenize_ast)?;

    let i_main = ast.get("MAIN").expect("No MAIN function found");

    println!("{:#?}", i_main);

    Ok(())
}
