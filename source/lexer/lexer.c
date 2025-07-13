#include "../../include/lexer/lexer.h"
#include "../../include/lexer/token.h"
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *Keywords[] = {
    "const", 
    "val",
    "mut",
    "struct",
    "enum",
    "impl",
    "interface",
    "priv",
    "pub",
    "override",
    "fn",
    "Self", "self",
    "if", "else",
    "switch",
    "for", 
    "break", 
    "continue",
    "unreachable"
};

Lexer LexerNew(char *source_name, char *source) {
    Lexer l;
    l.source = source;
    l.source_name = source_name;
    l.idx = 0;
    l.error_context = NULL;
    return l;
}

void *LexerNext(Lexer *l, Token *t) {
    char cur = l->source[l->idx];
    switch (cur) {
    case 0: t->tk = TK_EOF; break;
    default:
        LexerErrorContext(l, "Unknown char `%c`", cur);
        return NULL;
    }
}

void LexerErrorContext(Lexer *l, char *fmt, ...) {
    va_list vl;
    va_start(vl, fmt);

    if (l->error_context != NULL) free(l->error_context);
    l->error_context = (char *) malloc(sizeof(char *) * (strlen(fmt) + 256));
    vsprintf(l->error_context, fmt, vl);
}