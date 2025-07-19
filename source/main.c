// std c headers
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// canary headers
#include "../include/canary.h"
#include "../include/lexer/lexer.h"
#include "../include/lexer/token.h"
#include "../include/parser/parser.h"

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

    Lexer lexer = LexerNew(*file, buffer, file_size);
    Parser parser = ParserNew(&lexer);

    while (true) {
        void *node = ParserNext(&parser);
        if (node == NULL) {
            CanaryError(stderr, "Could not get node");
            if (parser.error_context != NULL) {
                CanaryContext(stderr, parser.error_context);
            }
            break;
        }
    };
    
    ParserFree(&parser);

    // TODO: Parse tokens

    // Cleanup
    fclose(fp);

    return 0;
}
