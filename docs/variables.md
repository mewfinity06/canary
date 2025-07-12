# Variables

## Form

Variables follow the form:

- <PROTECTION> %ident ':' <TYPE> '=' <EXPR> ';'
- <PROTECTION> %ident      ':='      <EXPR> ';'

## Examples

```rust
// Creating a mutable variable, explicit type
mut foo : i32 = 69;

// Creating an immutable variable, implicit type, can be made into mut
val bar := 420;

// Creating a const variable, type must be known at compile time, cannot be made into mut
const BAZ : str = "I am a string type!";

// Creating a static variable, type must be known at compile time
static TAU : f16 = 6.28;
```
