#include <stdio.h>
#include <stdlib.h>
#include "../../include/lexer/token.h"

char *TKName(enum TK tk) {
    switch (tk) {
    case TK_INVALID: return "TK_INVALID";
    case TK_EOF: return "TK_EOF";
    case TK_ASSIGN: return "TK_ASSIGN";
    case TK_PLUS_EQL: return "TK_PLUS_EQL";
    case TK_COLON: return "TK_COLON";
    case TK_SEMI_COLON: return "TK_SEMI_COLON";
    case TK_EQUAL: return "TK_EQUAL";
    case TK_PLUS: return "TK_PLUS";
    case TK_DOT: return "TK_DOT";
    case TK_COMMA: return "TK_COMMA";
    case TK_QUESION: return "TK_QUESION";
    case TK_O_BRACK: return "TK_O_BRACK";
    case TK_C_BRACK: return "TK_C_BRACK";
    case TK_O_PAREN: return "TK_O_PAREN";
    case TK_C_PAREN: return "TK_C_PAREN";
    case TK_O_SQUARE: return "TK_O_SQUARE";
    case TK_C_SQUARE: return "TK_C_SQUARE";
    case TK_IDENT: return "TK_IDENT";
    case TK_NUMBER: return "TK_NUMBER";
    case TK_KEYWORD: return "TK_KEYWORD";
    case TK_STRING: return "TK_STRING";
    case TK_RIGHT_ARROW: return "TK_RIGHT_ARROW";
    case TK_FAT_ARROW: return "TK_FAT_ARROW";
    case TK_DASH: return "TK_DASH";
    case TK_STAR: return "TK_STAR";
    case TK_SLASH: return "TK_SLASH";
    case TK_COMMENT: return "TK_COMMENT";
    case TK_PIPE: return "TK_PIPE";
    }
}

char *TokenFmt(Token token) {
    char *buffer = (char *) malloc(sizeof(char) * 256);
    sprintf(buffer, "Token %s => %s", TKName(token.tk), token.word);
    return buffer;
}

void TokenFree(Token *t) {
    free(t->word);
    free(t);
}