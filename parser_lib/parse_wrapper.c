#include <stdio.h>

#include "parser/src/parser.h"

#define SUCCESS 0
#define FAILURE 1

int parse(char* fileContents) {
    printf("%s\n", fileContents);
    test();
    printf("it should have ran by now");

    return SUCCESS;
}