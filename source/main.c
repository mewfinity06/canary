// std c headers
#include <stdbool.h>
#include <stdio.h>

// canary headers
#include "../include/canary.h"

// vendor header
#define FLAG_IMPLEMENTATION
#include "../vendor/flag.c"

#define NUM_EXPECTED_ARGS 1

void usage(FILE *stream) {
    fprintf(stream, "[USAGE] ./canary [OPTIONS]\n");
    fprintf(stream, "[OPTIONS]\n");
    flag_print_options(stream);
}

int main (int argc, char **argv) {
    bool  *help = flag_bool("help", false, "Displays this message!");
    char **file = flag_str("file", NULL, "File to read");

    if (!flag_parse(argc, argv)) {
        usage(stderr);
        flag_print_error(stderr);
        return 1;
    }

    if (*help) {
        usage(stdout);
        return 0;
    }

    if (!*file) {
        usage(stderr);
        CanaryError(stderr, "Must provide a file");
        return 1;
    }

    return 0;
}