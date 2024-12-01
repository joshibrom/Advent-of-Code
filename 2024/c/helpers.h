#pragma once
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

void ensureCommandParameters(int argc);

void expect(void* ptr, const char* msg);
void panic(const char* msg);

void readFile(const char* filename, char* buf);
void parseLines(const char* input, char** lineBuf, int nLines);


#ifndef HELPERS_IMPL
#define HELPERS_IMPL

void ensureCommandParameters(int argc) {
    if (argc != 2) panic("Incorrect number of command arguments supplied.");
}

void expect(void* ptr, const char* msg) {
    if (ptr == NULL) panic(msg);
}

void panic(const char* msg) {
    fprintf(stderr, "%s\n", msg);
    exit(EXIT_FAILURE);
}

void readFile(const char* filename, char* buf) {
    FILE* f = fopen(filename, "r");
    expect(f, "Could not open input file.");

    size_t idx = 0;
    while (!feof(f)) {
        buf[idx++] = fgetc(f);
    }

    fclose(f);
}

void parseLines(const char* input, char** lineBuf, int nLines) {
    char* toParse = malloc(strlen(input));
    expect(toParse, "Cannot create buffer to copy input to mutable string.");
    strcpy(toParse, input);
    char* tok = strtok(toParse, "\n");
    expect(tok, "Cannot allocate buffer to parse input file to lines.");
    int idx = 0;
    while (tok != NULL && idx < nLines) {
        lineBuf[idx++] = tok;
        tok = strtok(NULL, "\n");
    }
}

#endif