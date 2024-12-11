// {
//     "expr": "$price.foo.bar",
//     "dataset": "dataset5",
//     "bindings": {
//         "price": {
//             "foo": {
//                 "bar": 45
//             }
//         }
//     },
//     "result": 45
// }

// use jsonata_rs::{Lexer, Token, Result};

use jsonata_rs::{parse::expr, Lexer, Result};


fn main() -> Result<()> {
  let mut lexer = Lexer::new("price.foo.bar");
  let r = expr(&mut lexer)?;
  println!("{}", r.to_string());
  Ok(())
}
