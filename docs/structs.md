# Structs

## Form

### Struct

- 'const' %ident ':' 'struct' '=' '{' <STRUCTBODY> '}' ';'

### Impl

- %ident '+=' 'impl'        '{' <IMPLBODY> '}' ';'
- %ident '+=' 'impl' %ident '{' <IMPLBODY> '}' ';'

### Derives

- %ident '+=' %ident '{' <DERIVEBODY> '}' ';'

## Example

```rust
// Struct fields are `pub` by default (but can be explicit, if so chosen)
// To declare a private field (only accessible by it's `impl` block),
//    write `priv` before the field's name
const Person : struct = {
    name      : str,
    age       : u8,
    fav_color : (u8, u8, u8),
    // Not everyone has hair!
    hair_color? enum = {
        brown,
        black,
        red,
        blond,
        other: str,
    },
    priv ssid ? u32,
};

Person += PrettyPrint{};

Person += impl {
    pub const new : fn(
        name: str, 
        age: u8, 
        fav_color: (u8, u8, u8),
        // Since `Person.hair_color` is an anon enum, this is how you can access its type
        hair_color? Self.hair_color.type
        ssid? u32,
    ) -> Self = {.{
        .name       = name,
        .age        = age,
        .fav_color  = fav_color,
        .hair_color = hair_color,
        .ssid       = ssid,
    }};

    pub const who_am_i : fn(self) -> void = {
        printf("My name is {s} and I am {d} years old\n", self.name, self.age);
        printf("My favorite color is 0x{X} and ", self.color_to_hex());
        if (self.hair_color) |color| {
            printf("my hair color is 0x{X}\n", color);
        } else {
            printf("I do not have hair\n");
        }
    };

    const color_to_hex : fn(self) -> u8 = {
        // Implementation here
    }
};
```