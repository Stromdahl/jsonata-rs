⚠️ ⚠️ ⚠️ ⚠️  WORK IN PROGRESS ⚠️ ⚠️ ⚠️ ⚠️  

# JSONata

JSON query and transformation language

Rust port of the [JSONata query and transformation language](http://jsonata.org/).

* [JSONata in 5 minutes](https://www.youtube.com/embed/ZBaK40rtIBM)
* [JSONata language documentation](http://docs.jsonata.org/)
* [Try it out!](http://try.jsonata.org/)

## Installation

- TBD

## Quick start

In jsonata-rs/jsonata `cargo run --example jsonata`

```rust
use jsonata::{jsonata, Result};

fn main () -> Result<()> {
    let data = serde_json::json!({
         "example": [
             {"value": 4},
             {"value": 7},
             {"value": 13}
         ]
    });

    let expression = jsonata("$sum(example.value)")?;
    let result = expression.evaluate(&data)?;
    println!("{}", result); // Prints 24.0
    Ok(())
}
```

## More information
- JSONata [documentation](http://docs.jsonata.org/)
