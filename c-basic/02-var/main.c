#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

#define MAX_NUM1 1024

const int32_t MAX_NUM2 = 4096;

int main(int argc, char** argv) {
    int8_t a = 102;
    printf("a = %d\n", a);
    a = 88;
    printf("a = %d\n", a);

    printf("MAX_NUM1 = %d, MAX_NUM2 = %d\n", MAX_NUM1, MAX_NUM2);

    // MAX_NUM1 = 1025; // error: expression is not assignable
    // 1024 = 1025; // error: expression is not assignable

    // MAX_NUM2 = 5000; // error: cannot assign to variable 'MAX_NUM2' with const-qualified type 'const int32_t' (aka 'const int')

    return EXIT_SUCCESS;
}
