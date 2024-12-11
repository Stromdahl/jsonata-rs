use jsonata_rs::{evaluate, parse::expr, Lexer, Result};

fn main() -> Result<()> {
    let mut lexer = Lexer::new("-(4 * 4) + (4 * 4 * 2)");
    let r = expr(&mut lexer)?;
    let res = evaluate(r);
    println!("{}", res?.to_string());

    Ok(())
}
