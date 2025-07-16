#ifndef LEXER_H_
#define LEXER_H_

#include <stddef.h>
#include <stdbool.h>

#include "./token.h"

typedef struct Lexer Lexer;
struct Lexer {
    char  *source_name;
    char  *source;
    size_t source_len;
    size_t idx;

    char *error_context;
};

Lexer LexerNew            (char *source_name, char *source, size_t sourc);
bool  LexerNext           (Lexer *l, Token *t);
bool  LexerExpect         (Lexer *l, enum TK tk);
void  LexerErrorContext   (Lexer *l, char *fmt, ...);
bool  LexerSkipWhitespace (Lexer *lexer);
char  LexerPeek           (Lexer* lexer, size_t ahead);

bool readIdent  (Lexer *lexer, Token* token);
bool readNumber (Lexer *lexer, Token* token);
bool readString (Lexer *lexer, Token* token);
bool makeToken  (Lexer *lexer, Token* token, enum TK kind, size_t len);

#endif // LEXER_H_