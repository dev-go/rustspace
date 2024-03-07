#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdbool.h>

int main(int argc, char** argv) {
    printf("*** *** Array *** ***\n");

    {
        printf("--- --- --- ---\n");
        uint8_t len = 10;
        int32_t arr1[len];
        for (uint8_t i=0; i<len; i++) {
            printf("arr1[%u] = %d\n", i, arr1[i]);
        }
        printf("\n\n");

        // int64_t arr2[len] = {0}; // error: variable-sized object may not be initialized
        const uint8_t SIZE = 10;
        int64_t arr2[SIZE] = {0};
        for (uint8_t i=0; i<SIZE; i++) {
            printf("arr2[%u] = %lld\n", i, arr2[i]);
        }
        printf("\n\n");

        int64_t arr3[] = {1111, 2222, 3333, 4444, 5555};
        for (uint8_t i=0; i<5; i++) {
            printf("arr3[%u] = %lld\n", i, arr3[i]);
        }
        printf("\n\n");

        // int64_t* arr4 = {111, 222, 333, 444, 555}; // Segmentation fault:
        // for (uint8_t i=0; i<5; i++) {
        //     printf("arr4[%u] = %lld\n", i, arr4[i]);
        // }
        
        printf("arr1 size is: %lu\n", sizeof(arr1) / sizeof(int32_t));
        printf("arr2 size is: %lu\n", sizeof(arr2) / sizeof(int64_t));
        printf("arr3 size is: %lu\n", sizeof(arr3) / sizeof(int64_t));
        printf("\n\n");

        int64_t* e30 = &arr3[0];
        printf("arr3[0] = %p %lld\n", e30, *e30); 
        // printf("arr3[1] = %p %lld\n", e30 + sizeof(int64_t), *(e30 + sizeof(int64_t)));
        printf("arr3[1] = %p %lld\n", e30 + 1, *(e30 + 1));
        // printf("arr3[2] = %p %lld\n", e30 + 2 * sizeof(int64_t), *(e30 + 2 * sizeof(int64_t)));
        printf("arr3[2] = %p %lld\n", e30 + 2, *(e30 + 2));
    }

    {
        printf("--- --- --- ---\n");
        const uint8_t SIZE1 = 3;
        const uint8_t SIZE2 = 4;
        int32_t arr1[SIZE1][SIZE2];
        memset(arr1, 0, SIZE1 * SIZE2 * sizeof(int32_t));
        for (int i=0; i<SIZE1; i++) {
            for (int j=0; j<SIZE2; j++) {
                printf("arr1[%d][%d] = %d\t", i, j, arr1[i][j]);
            }
            printf("\n");
        }
        for (int i=0; i<SIZE1; i++) {
            for (int j=0; j<SIZE2; j++) {
                arr1[i][j] = (i+1) * 100 + (j+1);
            }
        }
        for (int i=0; i<SIZE1; i++) {
            for (int j=0; j<SIZE2; j++) {
                printf("arr1[%d][%d] = %d\t", i, j, arr1[i][j]);
            }
            printf("\n");
        }
    }

    {
        printf("--- --- --- ---\n");
        char s1[10];
        printf("s1 = %s, len=%lu, size=%lu\n", s1, strlen(s1), sizeof(s1));

        char s2[10] = "hello";
        printf("s2 = %s, len=%lu, size=%lu\n", s2, strlen(s2), sizeof(s2));

        char s3[10] = {'c', 'l', 'a', 'n', 'g'};
        printf("s3 = %s, len=%lu, size=%lu\n", s3, strlen(s3), sizeof(s3));

        char s4[10] = {'c', 'l', 'a', 'n', 'g', '\0'};
        printf("s4 = %s, len=%lu, size=%lu\n", s4, strlen(s4), sizeof(s4));

        // char* s5 = {'h', 'i', '\0'}; // Segmentation fault
        // printf("s5 = %s, len=%lu, size=%lu\n", s5, strlen(s5), sizeof(s5));

        char* s6 = &(s4[0]);
        printf("s6 = %s, len=%lu, size=%lu\n", s6, strlen(s6), sizeof(s6));

        s6[2] = '\0';
        printf("s6 = %s, len=%lu, size=%lu\n", s6, strlen(s6), sizeof(s6));
        printf("s4 = %s, len=%lu, size=%lu\n", s4, strlen(s4), sizeof(s4));
    }

    {
        printf("--- --- --- ---\n");
        char* a = "hello world";
        char* b = "hello clang";
        char c[] = "clang";
        printf("size: a = %lu, b = %lu, c = %lu\n", sizeof(a), sizeof(b), sizeof(c));
        const uint8_t LEN = 128;
        char buf[LEN];
        memset(buf, 0, LEN);
        strcat(buf, a);
        strcat(buf, "  ");
        strcat(buf, b);
        strcat(buf, "  ");
        strcat(buf, c);
        printf("buf = %s\n", buf);

        // char buf2[4];
        // memset(buf2, 0, 4);
        // strcat(buf2, a); // Abort trap: 6
        // printf("buf2 = %s\n", buf2);
    }

    {
        printf("--- --- --- ---\n");
	int32_t a = 10086;
        double b = 3.14;
        bool c = true;
        char d = 'X';
        char* e = "clang";
        char buf[128];
        memset(buf, 0, 128);
        sprintf(buf, "BUF: a=%d, b=%f, c=%d, d=%c, e=%s", a, b, c, d, e);
        printf("buf is\n%s\n", buf);
    }

    {
        printf("--- --- --- ---\n");
        char* fmt = "BUF: s=%4s, num=%d";
        char* str = "BUF: s=rust, num=10086";
        char s[4];
        char* sp = &s[0];
        int32_t num = 0;
        sscanf(str, fmt, sp, &num);
        printf("s = %s, num=%d\n", s, num);
        double d = atof("3.14");
        printf("atof eg: d = %f\n", d);
    }

    {
        printf("--- --- --- ---\n");
        char* s1 = "hello";
        char* s2 = "hello";
        char* s3 = "hello\0";
        char s4[] = "hello";
        char s5[] = "hello\0";
        printf("s1 = %s, s2 = %s, s3 = %s, s4 = %s, s5 = %s\n", s1, s2, s3, s4, s5);
        printf("s1 = %p, s2 = %p, s3 = %p, s4 = %p, s5 = %p\n", s1, s2, s3, s4, s5);
        printf("s1 == s2 : %d\n", s1 == s2);
        printf("s1 == s3 : %d\n", s1 == s3);
        printf("s1 == s4 : %d\n", s1 == s4);
        printf("s1 == s5 : %d\n", s1 == s5);
        printf("s4 == s5 : %d\n", s4 == s5); // warning: array comparison always evaluates to false
        printf("strcmp(s1, s2) : %s\n", strcmp(s1, s2) == 0 ? "equal": "not equal");
        printf("strcmp(s1, s3) : %s\n", strcmp(s1, s3) == 0 ? "equal": "not equal");
        printf("strcmp(s1, s4) : %s\n", strcmp(s1, s4) == 0 ? "equal": "not equal");
        printf("strcmp(s1, s5) : %s\n", strcmp(s1, s5) == 0 ? "equal": "not equal");
        printf("strcmp(s4, s5) : %s\n", strcmp(s4, s5) == 0 ? "equal": "not equal");
    }

    {
        printf("--- --- --- ---\n");
        char* a = "hello world";
        printf("a = %s\n", a);
        char* b = strchr(a, 'o');
        printf("b = %s\n", b);
        char* c = strrchr(a, 'o');
        printf("c = %s\n", c);

        char* d = strstr(a, "wo");
        printf("d = %s\n", d);
        
        char* e = strstr(a, "Wo");
        printf("e = %s\n", e);

        char* f = a + 5;
        printf("f = %s\n", f);

        char g[10];
        memset(g, 0, 10);
        strncpy(g, f, 3);
        printf("g = %s\n", g);
    }

    return EXIT_SUCCESS;
}
