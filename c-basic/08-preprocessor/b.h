#pragma once

// #ifndef B_H_
// #define B_H_

#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

#include "a.h"

int a_add(int a, int b);
int a_sub(int a, int b);

int b_add(int a, int b) {
    printf("[b_add] %d + %d = %d\n", a, b, a + b);
    a_add(a, b);
    return a + b;
}

int b_sub(int a, int b) {
    printf("[b_sub] %d - %d = %d\n", a, b, a - b);
    return a - b;
}

// #endif