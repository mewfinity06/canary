# Project: Canary

## About the Project

Canary is a statically typed, semi-type safe language written in Rust.

Here is an example "Hello world!" example

```canary
const main : fn() -> void = {
    printf("Hello world!");
};
```

Canary has expression based implicit returns like Rust and many functional languages.

There are 4 types of variable declarations.

```canary
// Mutable variable, explicit type
mut foo : i32 = 69;

// Immutable variable, implicit type
//   castable to 'mut'
let bar := 420;

// Const variable, type must be known at compile time
//   not castable to 'mut'
const Baz : str = "I am a string!s";

// Static variable, type must be known at compile time
//   immutable, castable to 'mut'
static Tau : f16 = Pi*2;
//   mutable
static mut Pi : f16 = 3.14159;
//   constant, not castable to 'mut'
static const Tau : f16 = Pi*2;
```

Structs, Enums, Macros, Interfaces, etc. follow the same syntax and all must be 'const' or 'static const'

```canary
const Person : struct = {
    name: str,
    age: u32,
};

// or

const Activity : enum = {
    Todo: str,
    SwimLaps: (u8, u8), // (laps, meters)
    Meditate: struct = {
        times: u8,
        seconds: f16,
    },
};

// and implementations upon structs and enums
Person += impl {
    // Methods and memebers here
};

// and derives on structs
Person += PrettyPrint{};
```

see [readme](./README.md) for more information on syntax

## Coding style

The Canary Project follows standard Rust conventions. After every time you finish a task, make sure to run `cargo fmt`

## Testing

Test files are in the [tests](./tests/) directory. In there, there will also be an `expected.json` file. To build any tests, use `cargo run -- run-tests` and to build `cargo run -- build-tests` and for both building and running, use `cargo run -- build-and-run-tests`. `cargo test` is not set up. If verboseness is needed, add the `-v` or `--verbose` flag.

## Git

Before staging any files, make sure to run the tests to see if the desired output is produced. Then, run `cargo fmt`, then you may stage the appropiate files.

Your commit message should follow conventional git messages.

## Specific workspace: `./lexer`

The lexer workspace is Canary's lexing module.

## Specific workspace: `./parser`

The parsing workspace is Canary's parsing module.

This is what we are currently working on.

## Specific workspace: `./utils`

The utils workspace is Canary's utility module.

The functions and macros here are used for the whole Canary project and must not have any direct knowledge of the other workspaces. These functions and macros are mainly for logging as of right now. This module should be in every other module's `Cargo.toml` as a dependency.

## Specific module: `./src`

This is where the main function is located in `main.rs`

Also included in this directory is:

### `cli.rs`

This is where the Cli struct lives, managed by cargo-clap

### `runner.rs`

This is where the main runner lives. So far, it just prints the `Node`s produced by `parser.program()`

### `tester.rs`

This is where the testing functions live that were described in the `Testing` section

