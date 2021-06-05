#include <stdio.h>

#include "lexer/lexer.h"

#define SUCCESS 0
#define FAILURE 1

int parser(char *fileContents, int charLen) {
    if (lexer(fileContents, charLen) == SUCCESS)
        return SUCCESS;
    return FAILURE;
}