#include <stdbool.h>

#include "helpers.h"

#define MAX_LEVELS 10

#define N_ROWS 1000
#define ROW_LEN 25

typedef struct {
    size_t nLevels;
    int levels[MAX_LEVELS];
} Report;

Report getReportFromLine(const char* line) {
    Report r = {0, {0}};
    char* p = malloc(strlen(line));
    expect(p, "Could not allocate buffer to parse line.");
    strcpy(p, line);
    char* tok = strtok(p, " ");
    while (tok != NULL) {
        r.levels[r.nLevels++] = atoi(tok);
        tok = strtok(NULL, " ");
    }
    free(p);
    return r;
}

typedef struct {
    size_t nErrors;
    size_t errorIndices[MAX_LEVELS];
} VerificationResult;

VerificationResult verifyReportDirection(const Report* r) {
    VerificationResult vr = {};
    const bool isIncr = r->levels[0] < r->levels[r->nLevels-1];
    for (size_t i = 0; i < r->nLevels - 1; ++i) {
        switch (isIncr) {
            case true:
                if (r->levels[i] >= r->levels[i+1])
                    vr.errorIndices[vr.nErrors++] = i;
                break;
            case false:
                if (r->levels[i] <= r->levels[i+1])
                    vr.errorIndices[vr.nErrors++] = i;
        }
    }
    return vr;
}

VerificationResult verifyReportGaps(const Report* r) {
    VerificationResult vr = {};
    for (size_t i = 0; i < r->nLevels - 1; ++i) {
        const int diff = abs(r->levels[i] - r->levels[i+1]);
        if (diff < 1 || diff > 3) {
            vr.errorIndices[vr.nErrors++] = i;
        }
    }
    return vr;
}

VerificationResult zipVerificationResults(const VerificationResult* v1, const VerificationResult* v2) {
    VerificationResult vr = {};
    bool v1Indicator[MAX_LEVELS] = {false};
    bool v2Indicator[MAX_LEVELS] = {false};
    for (size_t i = 0; i < v1->nErrors; ++i) {
        v1Indicator[i] = true;
    }
    for (size_t i = 0; i < v2->nErrors; ++i) {
        v2Indicator[i] = true;
    }
    for (size_t i = 0; i < MAX_LEVELS; ++i) {
        if (v1Indicator[i] || v2Indicator[i]) {
            vr.errorIndices[vr.nErrors++] = i;
        }
    }
    return vr;
}

size_t doP1(const char** lines, const size_t nLines) {
    Report reports[nLines];
    for (size_t i = 0; i < nLines; ++i) {
        reports[i] = getReportFromLine(lines[i]);
    }
    size_t passing = nLines;
    for (size_t i = 0; i < nLines; ++i) {
        const VerificationResult vrDir = verifyReportDirection(&reports[i]);
        const VerificationResult vrGap = verifyReportGaps(&reports[i]);
        VerificationResult vrZip = zipVerificationResults(&vrDir, &vrGap);
        if (vrZip.nErrors) passing--;
    }
    return passing;
}

size_t doP2(const char** lines, const size_t nLines) {
    Report reports[nLines];
    for (size_t i = 0; i < nLines; ++i) {
        reports[i] = getReportFromLine(lines[i]);
    }
    size_t passing = nLines;
    for (size_t i = 0; i < nLines; ++i) {
        const VerificationResult vrDir = verifyReportDirection(&reports[i]);
        const VerificationResult vrGap = verifyReportGaps(&reports[i]);
        VerificationResult vrZip = zipVerificationResults(&vrDir, &vrGap);
        if (vrZip.nErrors) passing--;
    }
    return passing;
}

int main(int argc, char* argv[]) {
    ensureCommandParameters(argc);
    char* inputBuf = malloc(N_ROWS * ROW_LEN);
    expect(inputBuf, "Could not allocate buffer to read file contents to.");
    readFile(argv[1], inputBuf);

    {
        char* testInput = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        const int nLines = 6;
        char* lines[nLines] = {};
        parseLines(testInput, lines, nLines);
        printf("[TEST] P1: %ld\n", doP1((const char**) lines, nLines));
        printf("[TEST] P2: %ld\n", doP2((const char**) lines, nLines));
    }

    char* lines[N_ROWS] = {};
    parseLines(inputBuf, lines, N_ROWS);
    free(inputBuf);
    printf("D02P01: %ld\n", doP1((const char**) lines, N_ROWS));

    return 0;
}