#pragma once
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/** 
 * Ensures that a given invocation of an AoC solution has 2 arguments:
 * 1. The program name.
 * 2. The input file path.
 * 
 * Will `panic` if these conditions are not met.
 */
void ensureCommandParameters(int argc);

/** 
 * Verifies that a given `ptr` is not `NULL`. `panic`s if some null pointer is
 * given with the supplied `msg` being printed to the terminal.
*/
void expect(const void* ptr, const char* msg);
/**
 * Prints the supplied `msg` to the terminal and exits with `EXIT_FAILURE`.
 */
void panic(const char* msg);

/**
 * Reads some input file `filename` to the pre-allocated `buf`.
 */
void readFile(const char* filename, char* buf);
/**
 * Parses lines from some `input` buffer to lines in `lineBuf` being
 * pre-allocated to have at most capacity for `nLines` lines.
 */
void parseLines(const char* input, char** lineBuf, int nLines);


#ifndef HELPERS_IMPL
#define HELPERS_IMPL

void ensureCommandParameters(int argc) {
    if (argc != 2) panic("Incorrect number of command arguments supplied.");
}

void expect(const void* ptr, const char* msg) {
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