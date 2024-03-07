#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

void hello(char* name, int age) {
    printf("hello, %s (age=%d)\n", name, age);
}

void show_array(int len, int32_t array[]) {
    printf("array: %p, size: %lu\n", array, sizeof(array)); // sizeof on array function parameter will return size of 'int32_t *'
    for (int i=0; i<len; i++) {
        printf("array[%d] = %d\n", i, array[i]);
    }
}

int main(int argc, char** argv) {
    printf("*** *** Pointer *** ***\n");

    {
        printf("--- --- --- ---\n");
        char* s1 = "hello";
        char s2[] = "hello";
        printf("s1: %p %s len=%lu, size=%lu\n", s1, s1, strlen(s1), sizeof(s1)); // s1: 0x10b31cf6a hello len=5, size=8
        printf("s2: %p %s len=%lu, size=%lu\n", s2, s2, strlen(s2), sizeof(s2)); // s2: 0x7ffee48e3a72 hello len=5, size=6

        uint64_t p = (uint64_t)s1;
        printf("p = %llu, p=0x%llx\n", p, p);
        char* s3 = (char*)p;
        printf("s3 = %s\n", s3);
    }

    {
        printf("--- --- --- ---\n");
        int32_t a = 100;
        int32_t* b = &a;
        printf("a: %p %d size: %lu\n", &a, a, sizeof(a));
        printf("b: %p %d size: %lu\n", b, *b, sizeof(b));
    }

    {
        printf("--- --- --- ---\n");
        int32_t* a = malloc(sizeof(int32_t));
        printf("a = %p %d\n", a, *a);
        *a = 100;
        printf("a = %p %d\n", a, *a);
        a[0] = 200;
        printf("a = %p %d\n", a, *a);
    }

    {
        printf("--- --- --- ---\n");
        int32_t* a = malloc(5 * sizeof(int32_t));
        a[0] = 101;
        a[1] = 102;
        a[2] = 103;
        a[3] = 104;
        a[4] = 105;
        for (int i=0; i<5; i++) {
            printf("Iterm[%d] = %d\n", i, *(a + i));
        }
    }

    {
        printf("--- --- --- ---\n");
        void(*f)(char*, int) = &hello;
        f("liu", 18);
        typedef void(*FUNC)(char*, int);
        FUNC f2 = &hello;
        f2("liu", 20);
    }

    {
        printf("--- --- --- ---\n");
        void* data = malloc(4);
        uint32_t* n = (uint32_t*)data;
        *n = 0x11223344;
        printf("n = %d, 0x%x\n", *n, *n);
        uint8_t* n0 = (uint8_t*)data;
        printf("n0 = %d, 0x%x\n", *n0, *n0); // n0 = 68, 0x44
        uint8_t* n1 = (uint8_t*)(data + 1);
        printf("n1 = %d, 0x%x\n", *n1, *n1); // n1 = 51, 0x33
    }

    {
        printf("--- --- --- ---\n");
        int32_t array[] = {11, 22, 33, 44, 55};
        printf("array: %p, size=%lu\n", array, sizeof(array));
        show_array(5, array);
    }

    return EXIT_SUCCESS;
}

