#ifndef CANARY_H_
#define CANARY_H_

#include <stdio.h>

void CanaryInfo    (FILE *stream, char *fmt, ...);
void CanaryWarning (FILE *stream, char *fmt, ...);
void CanaryError   (FILE *stream, char *fmt, ...);
void CanaryContext (FILE *stream, char *fmt, ...);

#endif // CANARY_H_