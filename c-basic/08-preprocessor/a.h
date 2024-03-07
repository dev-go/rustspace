#pragma once

// #ifndef A_H_
// #define A_H_

#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

#include "b.h"

int b_add(int a, int b);
int b_sub(int a, int b);

int a_add(int a, int b) {
    printf("[a_add] %d + %d = %d\n", a, b, a + b);
    return a + b;
}

int a_sub(int a, int b) {
    printf("[a_sub] %d - %d = %d\n", a, b, a - b);
    b_sub(a, b);
    return a - b;
}

// #endif