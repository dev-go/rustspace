#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

#include "a.h"
#include "b.h"

int calc_add(int a, int b) {
    return a + b;
}

int calc_sub(int a, int b) {
    return a - b;
}


int main(int argc, char** argv) {
    printf("*** *** Preprocessor *** ***\n");
    
    {
        printf("--- --- --- ---\n");
        #define NUM 2
        printf("NUM = %d\n", NUM);

        // #define OS "MAC"
        // clang main.c -DOS=\"MACOS\" -DPLATFORM=2
        printf("OS = %s\n", OS);
    }

    {
        printf("--- --- --- ---\n");
        #define LINUX 1
        #define MACOS 2
        #define WINDOWS 3

        // clang main.c -DOS=\"MACOS\" -DPLATFORM=2
        if (PLATFORM == LINUX) {
            printf("platform is linux\n");
        } else if (PLATFORM == MACOS) {
            printf("platform is macos\n");
        } else if (PLATFORM == WINDOWS) {
            printf("platform is windows\n");
        } else {
            printf("platform is unknown (%d)\n", PLATFORM);
        }

        int a = 10;
        int b = 2;
        printf("==> a_add: %d\n", a_add(a, b));
        printf("==> b_add: %d\n", b_add(a, b));
        printf("==> a_sub: %d\n", a_sub(a, b));
        printf("==> b_sub: %d\n", b_sub(a, b)); 
    }

    {
        printf("--- --- --- ---\n");
        #define MAX(A,B) A>B?A:B
        int a = 88;
        int b = 66;
        int c = 100;
        printf("MAX(a, b) = %d\n", MAX(a, b));
        printf("MAX(a, c) = %d\n", MAX(a, c));
        double d = 3.14;
        double e = 5.01;
        printf("MAX(d, e) = %f\n", MAX(d, e));
    }

    {
        printf("--- --- --- ---\n");
        #define LOOP(FROM,TO,FUNC) \
            for(int i=FROM; i<TO; i++) { \
                FUNC;\
            }
        int32_t arr[] = {101, 102, 103, 104, 105, 106, 107, 108};
        LOOP(2, 6, printf("arr[%d] = %d\n", i, arr[i]));
    }

    {
        printf("--- --- --- ---\n");
        #define calc(NAME,A,B) calc_##NAME(A, B)
        int a = 100;
        int b = 88;
        int c = calc(add, a, b);
        int d = calc(sub, a, b);
        printf("c = %d, d = %d\n", c, d);
    }

    {
        printf("--- --- --- ---\n");
        #define LOG(LEVEL,FMT,...) printf(LEVEL);printf(FMT, __VA_ARGS__);printf("\n");
        #define ERROR(FMT,...) LOG("[ERROR]: ",FMT,__VA_ARGS__)
        ERROR("invalid parameter: a = %d", 55);
    }

    return EXIT_SUCCESS;
}
