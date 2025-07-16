#ifndef CANARY_H_
#define CANARY_H_

#include <stdio.h>

#define INFO    "\x1b[38;5;40m"
#define WARNING "\x1b[38;5;226m"
#define ERROR   "\x1b[38;5;196m"
#define CONTEXT "\x1b[38;5;87m"
#define RESET   "\x1b[0m"

void CanaryInfo    (FILE *stream, char *fmt, ...);
void CanaryWarning (FILE *stream, char *fmt, ...);
void CanaryError   (FILE *stream, char *fmt, ...);
void CanaryContext (FILE *stream, char *fmt, ...);

#endif // CANARY_H_