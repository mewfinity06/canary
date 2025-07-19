#include "../../include/parser/parser.h"
#include <stdlib.h>
#include <stdarg.h>
#include <string.h>
#include <stddef.h>
#include <stdio.h>

#define ERROR_CONTEXT_BUFFER 256

Parser ParserNew(Lexer *lexer) {
    Parser parser;
    parser.lexer = lexer;
    parser.cur = (Token *) malloc(sizeof(Token));
    parser.cur->tk = TK_INVALID;
    parser.peek = (Token *) malloc(sizeof(Token));
    parser.peek->tk = TK_INVALID;
    parser.error_context = NULL;
    return parser;
}

void *ParserNext(Parser *parser) {
    ParserErrorContext(parser, "ParserNext not implemented");
    return NULL;
}

void ParserFree(Parser *parser) {
    if (parser->error_context != NULL) free(parser->error_context);
    LexerFree(parser->lexer);
}


void ParserErrorContext(Parser *parser, char *fmt, ...) {
    va_list vl;
    va_start(vl, fmt);

    if (parser->error_context != NULL) free(parser->error_context);
    parser->error_context = (char*) malloc(sizeof(char *) * (strlen(fmt) + ERROR_CONTEXT_BUFFER));
    vsprintf(parser->error_context, fmt, vl);
}
