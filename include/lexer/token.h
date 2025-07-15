#ifndef TOKEN_H_
#define TOKEN_H_

enum TK {
    TK_INVALID,
    TK_EOF,

    // 2 char tokens
    TK_ASSIGN, // :=
    TK_PLUS_EQL, // +=

    // 1 char tokens
    TK_COLON, // :
    TK_SEMI_COLON, // ;
    TK_EQUAL, // =
    TK_PLUS, // +
    TK_DOT, // .
    TK_COMMA, // ,
    TK_QUESION, // ?
    TK_O_BRACK, // {
    TK_C_BRACK, // }
    TK_O_PAREN, // (
    TK_C_PAREN, // )
    TK_O_SQUARE, // [
    TK_C_SQUARE, // ]

    // Literals
    TK_IDENT,
    TK_KEYWORD,
    TK_STRING,
};

char *TKName(enum TK tk);

typedef struct Token Token;
struct Token {
    char *word;
    enum TK tk;
};

#define NewToken(t) \
    Token *(t) = (Token *) malloc(sizeof(Token)); \
    t->word = "INVALID"; \
    t->tk = TK_INVALID

char *TokenFmt(Token t);
#endif // TOKEN_H_