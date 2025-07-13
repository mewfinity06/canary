#pragma once
#include <stdio.h>

void CanaryInfo    (FILE *stream, char *fmt, ...);
void CanaryWarning (FILE *stream, char *fmt, ...);
void CanaryError   (FILE *stream, char *fmt, ...);
void CanaryContext (FILE *stream, char *fmt, ...);