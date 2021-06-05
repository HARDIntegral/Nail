#include <stdio.h>

#define SUCCESS 0
#define FAILURE 1

int lexer(char *fileContents, int charLen) {
    for (int i = 0; i < charLen; i++) {
        char tmp = *(fileContents + i);
        if (tmp == ' ')
            tmp = '#';
        printf("%c", tmp);
    }
    printf("\n");
    return SUCCESS;
}