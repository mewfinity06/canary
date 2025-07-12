# Interface

## Form

### Interface

- 'const' %ident ':' 'interface' '=' '{' <INTERFACEBODY> '}' ';'

### Derives

- %ident '+=' %ident '{' <DERIVEBODY> '}' ';'

## Example

```rust
const MyInterface : interface = {
    const say_hello : fn(self) -> void = {
        print("Hello from {s}", self.type);
    };
    const override_me : fn(self) -> str = { "I wish I was a different string" };
    const implement_me : fn(self, x: u32, y:u32) -> u32;
};

const MyType : struct = {};

MyType += MyInterface {
    // say_hello is already implemented
    override const override_me : fn(self) -> str = { "Aw hell yeah!" };
    const implement_me : fn(self, x: u32, y: u32) -> u32 = { y * 2 - 3 + x^x };
};
```