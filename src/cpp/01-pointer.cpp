#include <stdio.h>

int main() {
    int x; 
    int *p; // note its genuinely int *p not int* p;

    x = 5;
    p = &x; // p points to x

    printf("variable x: %d\n", x);

    printf("address of x: %p\n", p);  

    return 0;
}

// printf formatters: 
// %p - pointer, %d/%i - int, %s - string, %u - unsigned, %f - float, %c - character