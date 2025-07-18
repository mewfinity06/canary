#ifndef TOKEN_H_
#define TOKEN_H_

enum TK {
    TK_INVALID,
    TK_EOF,

    // 3 char tokens
    TK_3_DOT, // ...

    // 2 char tokens
    TK_ASSIGN, // :=
    TK_PLUS_EQL, // +=
    TK_MINUS_EQL, // -=
    TK_STAR_EQL, // *=
    TK_SLASH_EQL, // /=
    TK_LESS_EQL, // <=
    TK_GREATER_EQL, // >=
    TK_RIGHT_ARROW, // ->
    TK_FAT_ARROW, // =>
    TK_PIPE, // |>


    // 1 char tokens
    TK_COLON, // :
    TK_SEMI_COLON, // ;
    TK_EQUAL, // =
    TK_PLUS, // +
    TK_DASH, // -
    TK_STAR, // *
    TK_SLASH, // /
    TK_VERT_BAR, // |
    TK_DOT, // .
    TK_COMMA, // ,
    TK_LESS, // <
    TK_GREATER, // >
    TK_QUESION, // ?
    TK_BANG, // !
    TK_POUND, // #
    TK_O_BRACK, // {
    TK_C_BRACK, // }
    TK_O_PAREN, // (
    TK_C_PAREN, // )
    TK_O_SQUARE, // [
    TK_C_SQUARE, // ]

    // Literals
    TK_IDENT,
    TK_NUMBER,
    TK_KEYWORD,
    TK_STRING,
    TK_COMMENT,
};

char *TKName(enum TK tk);

typedef struct Token Token;
struct Token {
    char *word;
    enum TK tk;
};

#define NewToken(t) \
    Token *(t) = (Token *) malloc(sizeof(Token)); \
    t->word = NULL; \
    t->tk = TK_INVALID

char *TokenFmt(Token t);
void  TokenFree(Token *t);
#endif // TOKEN_H_
