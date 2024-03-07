#include <stdlib.h>
#include <stdio.h>

#include "calc.h"

int32_t add(int32_t a, int32_t b) {
    printf("[add] %d + %d = %d\n", a, b, a + b);
    return a + b;
}

int32_t sub(int32_t a, int32_t b) {
    printf("[sub] %d - %d = %d\n", a, b, a - b);
    return a - b;
}
