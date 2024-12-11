use jsonata_rs::Result;
use jsonata_rs::jsonata;

fn main() -> Result<()> {
    // let mut lexer = Lexer::new("-(4 * 4) + (4 * 4 * 2)");
    // let r = parse(&mut lexer)?;
    // let res = evaluate(r);

    let expression = jsonata("-(4 * 4) + (4 * 4 * 2) % 4")?;
    let _ = expression.evaluate();
    // println!("{}", res?.to_string());

    Ok(())
}
