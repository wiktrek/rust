// gcc main.c -o main
#include <stdio.h>
#include <stdlib.h>
int main() {
    char name[32];
    printf("Hello world!\n");
    printf("main.c\n");
    fgets(name, 32, stdin);
    printf("Your name is %s", name);
    return 0;
}