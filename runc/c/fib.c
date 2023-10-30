#include <stdio.h>
#include <stdlib.h>
int main() {
    int x, y,z;
    x = 1;
    y = 0;
    do {
        printf("%d \n", x);
        z = x + y;
        y = x;
        x = z;
    } while (x < 1024);
}