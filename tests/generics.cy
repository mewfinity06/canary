const printf := std.c.printf;

// ...snip from Canary stdlib... //
const Result : enum<T, E> = {
    Ok: T,
    Error: E,
};

const return_error : fn() -> Result<str, str> = {
    Result.Error { "I am an error!" }
};

const error_wrapper : fn() -> Result<str, str> = {
    return_error()?
}

const main : fn() -> void = {
    switch (return_error()) {
        .Ok    : |ok| => {
            printf("Ok message: {s}\n", ok);
        },
        .Error : |err| => {
            printf("Error message: {s}\n", err);
        },
    }
};