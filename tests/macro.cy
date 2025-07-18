const my_printf : macro = {
    () := { unreachable };
    (fmt: str) := {
        std.c.printf(#fmt);
    };
    // Variadics are only allowed in macros
    (fmt: str, args: ...) := {
        std.c.printf(#fmt, #args);
    };
};

const main : fn() = {
    // Valid!
    my_printf!("Hello, {s}! I am a {s}!\n", "user", "macro");
    // Also valid
    my_printf!("Omg, no more args??\n");
    // Not valid
    my_printf!();
};
