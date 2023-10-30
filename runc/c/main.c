// gcc main.c -o main
#include <stdio.h>
#include <stdlib.h>
int main() {
    char name[32];
    printf("Hello world!\n");
    printf("main.c\n");
    scanf("%s", &name);
    printf("Your name is %s", name);
    return 0;
}