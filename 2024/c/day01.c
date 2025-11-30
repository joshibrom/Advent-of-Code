#include <math.h>
#include <stdio.h>
#include <string.h>

#include "helpers.h"

#define N_LINES 1000
#define LINE_LEN 16

typedef struct {
    int c1;
    int c2;
} LineValues;

LineValues parseLine(const char* line) {
    LineValues lv;
    sscanf(line, "%d   %d", &lv.c1, &lv.c2);
    return lv;
}

int sortComp(const void* a, const void* b) {
    return *(int*)a - *(int*)b;
}

int doP1(int* col1, int* col2) {
    int dist = 0;
    for (int i = 0; i < N_LINES; ++i) {
        dist += abs(col1[i] - col2[i]);
    }
    return dist;
}

int doP2(int* col1, int* col2) {
    int sim = 0;
    for (int i = 0; i < N_LINES; ++i) {
        int occs = 0;
        for (int j = 0; j < N_LINES; ++j) {
            if (col1[i] == col2[j]) occs++;
            if (col1[i] < col2[j]) break;
        }
        sim += occs * col1[i];
    }
    return sim;
}

int main(int argc, char* argv[]) {
    ensureCommandParameters(argc);

    char* inputBuf = malloc(N_LINES * LINE_LEN);
    expect(inputBuf, "Could not create buffer to read input file.");
    readFile(argv[1], inputBuf);

    char* lines[N_LINES] = {};
    parseLines(inputBuf, lines, N_LINES);
    free(inputBuf);

    int col1[N_LINES];
    int col2[N_LINES];
    for (int i = 0; i < N_LINES; ++i) {
        LineValues lv = parseLine(lines[i]);
        col1[i] = lv.c1;
        col2[i] = lv.c2;
    }
    qsort(col1, N_LINES, sizeof(int), sortComp);
    qsort(col2, N_LINES, sizeof(int), sortComp);

    printf("D01P01: %d\n", doP1(col1, col2));
    printf("D01P02: %d\n", doP2(col1, col2));

    return 0;
}