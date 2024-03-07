#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdarg.h>

#include "calc.h"

void hello(char* name, uint8_t age);

char* hi(char* name) {
    uint32_t len = sizeof(name) + 10;
    // char buf[len]; // warning: address of stack memory associated with local variable 'buf' returned
    char* buf = malloc(len * sizeof(char));
    memset(buf, 0, len);
    sprintf(buf, "hello, %s!", name);
    return buf;
}

int32_t my_sum(int32_t count, ...);

int main(int argc, char** argv) {
    printf("*** *** Func *** ***\n");

    {
        printf("--- --- --- ---\n");
        printf("argc = %d\n", argc);
        for (int i=0; i<argc; i++) {
            printf("argv[%d] = %s\t", i, argv[i]);
        }
        printf("\n");
    }

    {
        printf("--- --- --- ---\n");
        char* name = "liu";
        uint8_t age = 18;
        hello(name, age);
        char* result = hi(name);
        printf("[hi] result: %s\n", result);
    }

    {
        printf("--- --- --- ---\n");
        int32_t result = my_sum(3, 11, 22, 33);
        printf("my_sum: result = %d\n", result);


        int32_t add_result = add(11, 222);
        int32_t sub_result = sub(11, 222);
        printf("add_result = %d, sub_result = %d\n", add_result, sub_result);
    }

    return EXIT_SUCCESS;
}

void hello(char* name, uint8_t age) {
    printf("hello: name=%s(age=%d)\n", name, age);
}

int32_t my_sum(int32_t count, ...) {
    int32_t result = 0;
    va_list args;
    va_start(args, count);
    for (int i=0; i<count; i++) {
        int32_t arg = va_arg(args, int32_t);
        result += arg;
        printf("arg: %d, %d, result=%d\n", i, arg, result);
    }
    va_end(args);
    return result;
}
