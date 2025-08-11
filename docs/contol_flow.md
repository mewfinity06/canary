# Control Flow

## If

- 'if' '(' <IFCONDITION> ')' '{' <IFBODY> '}' ('else' '{' <IFBODY> '}')?
- 'if' '(' <IFCONDITION> ')' '{' <IFBODY> '}' ('else' <IF>)?
- 'if' '(' <IFCONDITION> ')' ':' '|' <CAPTUREBODY> '|' '{' <IFBODY> '}' ('else' '{' <IFBODY> '}')?
- 'if' '(' <IFCONDITION> ')' ':' '|' <CAPTUREBODY> '|' '{' <IFBODY> '}' ('else' <IF>)?

## Switch

- 'switch' '(' <SWITCHCONDITION> ')' '{' <SWITCHBODY> '}'

## Examples

```rust
// If example
if (2 > 3) { 
    printf("2 is bigger than 3\n"); 
} else {
    printf("2 is NOT bigger than 3\n");
}

const am_i_null : ?u8 = null;

if (am_i_null) : |item| {
    printf("`{d}` is not null!\n");
}

// Switch example

const Activity : enum {
    Todo: str,
    SwimLaps: i8,
    Meditate: struct {
        times: i8,
        seconds: f16,
    },
    NoMore,
};

const do_activity : fn(activity: Activity) -> void = {
    switch (activity) {
        .Todo     : |todo| => { printf("TODO: {s}\n", todo); },
        .SwimLaps : |laps| => printf("Swim {d} laps\n", laps),
        .Meditate : |{times, seconds}|  => {
            printf("Meditate {d} times for {f} seconds\n", times, seconds);
        },
        .NoMore => return,
    };
};
```