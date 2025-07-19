// std c headers
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// canary headers
#include "../include/canary.h"
#include "../include/lexer/lexer.h"
#include "../include/lexer/token.h"

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
    // Parse flags
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

    // Read file
    FILE *fp;
    fp = fopen(*file, "r");
    if (fp == NULL) {
        CanaryError(stderr, "Could not open file `%s`", *file);
        return 1;
    }

    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    char buffer[file_size+1];

    size_t bytes_read = fread(buffer, 1, file_size, fp);
    if ((long)bytes_read != file_size) {
        CanaryError(stderr, "Error reading file: expected %ld bytes, read %zu\n", file_size, bytes_read);
        fclose(fp);
        return 1;
    }
    buffer[file_size] = '\0';

    Lexer l = LexerNew(*file, buffer, file_size);
    NewToken(t);
    while (true) {
        if (!LexerNext(&l, t)) {
            CanaryError(stderr, "Could not get token.");
            if (l.error_context != NULL) {
                CanaryContext(stderr, l.error_context);
            }
            return 0;
        }
        char *format = TokenFmt(*t);
        CanaryInfo(stdout, "Found %s", format);
        free(format);
        if (t->tk == TK_EOF || t->tk == TK_INVALID) {
            break;
        }
    }
    TokenFree(t);
    LexerFree(&l);

    // TODO: Parse tokens

    // Cleanup
    fclose(fp);

    return 0;
}
