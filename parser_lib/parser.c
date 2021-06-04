#include <stdio.h>

#include "lexer/lexer.h"

#define SUCCESS 0
#define FAILURE 1

int parser(char* fileContents) {
    if (lexer(fileContents) == SUCCESS)
        return SUCCESS;
    return FAILURE;
}