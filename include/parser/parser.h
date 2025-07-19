#ifndef PARSER_H_
#define PARSER_H_

#include "../../include/lexer/lexer.h"
#include "../../include/lexer/token.h"

typedef struct Parser Parser;
struct Parser {
    Lexer *lexer;
    Token *cur;
    Token *peek;

    char *error_context;
};

Parser ParserNew          (Lexer *lexer);
void   ParserFree         (Parser *parser);
void  *ParserNext         (Parser *parser);
void   ParserAdvance      (Parser *parser);
void   ParserErrorContext (Parser *parser, char *fmt, ...);

#endif // PARSER_H_
