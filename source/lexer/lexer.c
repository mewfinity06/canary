#include "../../include/lexer/lexer.h"
#include "../../include/lexer/token.h"
#include <ctype.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ERROR_CONTEXT_BUFFER 256

#define KEYWORD_LEN (sizeof(Keywords)/sizeof(Keywords[0]))
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

bool isKeyword(char* needle) {
    for (size_t i = 0; i < KEYWORD_LEN; ++i) {
        if (strcmp(needle, Keywords[i]) == 0) return true;
    }
    return false;
}

Lexer LexerNew(char *source_name, char *source, size_t source_len) {
    Lexer lexer;
    lexer.source = source;
    lexer.source_name = source_name;
    lexer.source_len = source_len;
    lexer.idx = 0;
    lexer.error_context = NULL;
    return lexer;
}

bool LexerNext(Lexer *lexer, Token *token) {
    if (!LexerSkipWhitespace(lexer)) return false;
    char cur = lexer->source[lexer->idx];
    switch (cur) {
    case 'a'...'z':
    case 'A'...'Z': if (!readIdent(lexer, token))  return false; break;
    case '0'...'9': if (!readNumber(lexer, token)) return false; break;
    case '"':       if (!readString(lexer, token)) return false; break;
    case ':': switch (LexerPeek(lexer, 1)) {
        case '=': if (!makeToken(lexer, token, TK_ASSIGN, 2)) return false; break;
        default:  if (!makeToken(lexer, token, TK_COLON, 1)) return false; break;
    } break;
    case '=': if (!makeToken(lexer, token, TK_EQUAL, 1)) return false; break;
    case '.': if (!makeToken(lexer, token, TK_DOT, 1)) return false; break;
    case ';': if (!makeToken(lexer, token, TK_SEMI_COLON, 1)) return false; break;
    case '(': if (!makeToken(lexer, token, TK_O_PAREN, 1)) return false; break;
    case ')': if (!makeToken(lexer, token, TK_C_PAREN, 1)) return false; break;
    case '{': if (!makeToken(lexer, token, TK_O_BRACK, 1)) return false; break;
    case '}': if (!makeToken(lexer, token, TK_C_BRACK, 1)) return false; break;
    case '[': if (!makeToken(lexer, token, TK_O_SQUARE, 1)) return false; break;
    case ']': if (!makeToken(lexer, token, TK_C_SQUARE, 1)) return false; break;
    case 0: token->tk = TK_EOF; break;
    default:
        LexerErrorContext(lexer, "Unknown char `%c`", cur);
        return false;
    }
    return true;
}

bool LexerSkipWhitespace (Lexer *lexer) {
    while (lexer->idx < lexer->source_len && isspace(lexer->source[lexer->idx])) lexer->idx++;
    return true;
}

void LexerErrorContext(Lexer *lexer, char *fmt, ...) {
    va_list vl;
    va_start(vl, fmt);

    if (lexer->error_context != NULL) free(lexer->error_context);
    lexer->error_context = (char *) malloc(sizeof(char *) * (strlen(fmt) + ERROR_CONTEXT_BUFFER));
    vsprintf(lexer->error_context, fmt, vl);
}

char LexerPeek (Lexer* lexer, size_t offset) {
    if (lexer->idx + offset < lexer->source_len) {
        return lexer->source[lexer->idx + offset];
    }
    return 0;
}

bool readIdent(Lexer *lexer, Token *token) {
    size_t start = lexer->idx;
    while (lexer->idx < lexer->source_len && isalnum(lexer->source[lexer->idx])) {
        lexer->idx++;
    }
    size_t size = lexer->idx - start;
    if (token->word != NULL) {
        free(token->word);
        token->word = NULL;
    }
    token->word = (char *)malloc(sizeof(char) * (size + 1));
    if (token->word == NULL) return false;
    strncpy(token->word, lexer->source + start, size);
    token->word[size] = '\0';
    if (isKeyword(token->word)) {
        token->tk = TK_KEYWORD;
    } else {
        token->tk = TK_IDENT;
    }
    return true;
}

bool readNumber(Lexer *lexer, Token *token) {
    size_t start = lexer->idx;
    while (lexer->idx < lexer->source_len && isdigit(lexer->source[lexer->idx])) {
        lexer->idx++;
    }
    // Basic support for floating point numbers
    if (lexer->idx < lexer->source_len && lexer->source[lexer->idx] == '.') {
        lexer->idx++;
        while (lexer->idx < lexer->source_len && isdigit(lexer->source[lexer->idx])) {
            lexer->idx++;
        }
    }
    size_t size = lexer->idx - start;
    if (token->word != NULL) {
        free(token->word);
        token->word = NULL;
    }
    token->word = (char *)malloc(sizeof(char) * (size + 1));
    if (token->word == NULL) return false;
    strncpy(token->word, lexer->source + start, size);
    token->word[size] = '\0';
    token->tk = TK_NUMBER;
    return true;
}

bool readString(Lexer *lexer, Token *token) {
    lexer->idx++; // Consume the opening quote
    size_t start = lexer->idx;
    while (lexer->idx < lexer->source_len && lexer->source[lexer->idx] != '"' && lexer->source[lexer->idx] != '\0') {
        // Basic handling for escaped quotes within the string (e.g., ignoring them for now)
        if (lexer->source[lexer->idx] == '\\' && lexer->idx + 1 < lexer->source_len) {
            lexer->idx++; // Skip escaped character
        }
        lexer->idx++;
    }

    if (lexer->idx >= lexer->source_len || lexer->source[lexer->idx] == '\0') {
        LexerErrorContext(lexer, "Unterminated string literal");
        return false;
    }

    size_t size = lexer->idx - start;
    if (token->word != NULL) {
        free(token->word);
        token->word = NULL;
    }
    token->word = (char *)malloc(sizeof(char) * (size + 1));
    if (token->word == NULL) return false;
    strncpy(token->word, lexer->source + start, size);
    token->word[size] = '\0';
    lexer->idx++; // Consume the closing quote
    token->tk = TK_STRING;
    return true;
}

bool makeToken (Lexer *lexer, Token* token, enum TK kind, size_t len) {
    if (token->word != NULL) {
        free(token->word);
        token->word = NULL;
    }
    token->word = (char *)malloc(sizeof(char) * (len + 1));
    if (token->word == NULL) {
        lexer->error_context = "Memory allocation failed for token word";
        return false;
    }

    strncpy(token->word, lexer->source + lexer->idx, len);
    token->word[len] = '\0';

    token->tk = kind;
    lexer->idx += len;
    return true;
}