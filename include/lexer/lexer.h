#ifndef LEXER_H_
#define LEXER_H_

#include <stddef.h>
#include <stdbool.h>

#include "./token.h"

typedef struct Lexer Lexer;
struct Lexer {
    char  *source_name;
    char  *source;
    size_t idx;

    char *error_context;
};

Lexer  LexerNew(char *source_name, char *source);
Token *LexerNext(Lexer *l, Token *t);
bool   LexerExpect(Lexer *l, enum TK tk);
void   LexerErrorContext(Lexer *l, char *fmt, ...);

char *readIdent(Lexer *lexer);
char *readNumber(Lexer *lexer);
char *readString(Lexer *lexer);

#endif // LEXER_H_