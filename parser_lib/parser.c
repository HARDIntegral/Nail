#include <stdio.h>

#include "test.h"

#define SUCCESS 0
#define FAILURE 1

int parser(char* fileContents) {
    printf("%s\n", fileContents);
    test();
    return SUCCESS;
}