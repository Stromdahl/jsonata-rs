⚠️ ⚠️ ⚠️ ⚠️  WORK IN PROGRESS ⚠️ ⚠️ ⚠️ ⚠️  

# JSONata

JSON query and transformation language

## Quick start

In Node.js:

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
