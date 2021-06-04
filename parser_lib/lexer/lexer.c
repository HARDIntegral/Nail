#include <stdio.h>

#define SUCCESS 0
#define FAILURE 1

int lexer(char* fileContents) {
    printf("%s\n", fileContents);
    return SUCCESS;
}