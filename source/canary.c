// std c headers
#include <stdio.h>
#include <stdarg.h>

// canary headers
#include "../include/canary.h"


void CanaryInfo (FILE* stream, char *fmt, ...) {
    va_list vl;
    va_start(vl, fmt);
    fprintf(stream, "["INFO"INFO"RESET"] ");
    vfprintf(stream, fmt, vl);
    fprintf(stream, "\n");
}

void CanaryWarning (FILE* stream, char *fmt, ...) {
    va_list vl;
    va_start(vl, fmt);
    fprintf(stream,"["WARNING"WARNING"RESET"] ");
    vfprintf(stream,fmt, vl);
    fprintf(stream,"\n");
}

void CanaryError (FILE* stream, char *fmt, ...) {
    va_list vl;
    va_start(vl, fmt);
    fprintf(stream, "["ERROR"ERROR"RESET"] ");
    vfprintf(stream, fmt, vl);
    fprintf(stream, "\n");
}

void CanaryContext (FILE* stream, char *fmt, ...) {
    va_list vl;
    va_start(vl, fmt);
    fprintf(stream, "["CONTEXT"CONTEXT"RESET"] ");
    vfprintf(stream, fmt, vl);
    fprintf(stream, "\n");
}