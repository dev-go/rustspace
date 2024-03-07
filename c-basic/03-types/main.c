#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdbool.h>

int main(int argc, char** argv) {
    {
        printf("int8_t size is: %lu\n", sizeof(int8_t));
        printf("uint8_t size is: %lu\n", sizeof(uint8_t));
        printf("int16_t size is: %lu\n", sizeof(int16_t));
        printf("uint16_t size is: %lu\n", sizeof(uint16_t));
        printf("int32_t size is: %lu\n", sizeof(int32_t));
        printf("uint32_t size is: %lu\n", sizeof(uint32_t));
        printf("int64_t size is: %lu\n", sizeof(int64_t));
        printf("uint64_t size is: %lu\n", sizeof(uint64_t));
    }

    {
        int8_t a = -88;
        uint8_t b = 66;
        int16_t c = -1000;
        uint16_t d = 1024;
        int32_t e = -2000000000;
        uint32_t f = 4000000000;
        int64_t g = -10000000000;
        uint64_t h = 20000000000;
        printf("a=%d, b=%u, c=%d, d=%u, e=%d, f=%u, g=%lld, h=%llu\n", a, b, c, d, e, f, g, h);
    }

    {
        int8_t a = 0b1001;
        int8_t b = 010;
        int8_t c = 0x10;
        printf("a = %d 0o%o 0x%x\n", a, a, a);
        printf("b = %d 0o%o 0x%x\n", b, b, b);
        printf("c = %d 0o%o 0x%x\n", c, c, c);
    }

    {
        printf("float size is: %lu\n", sizeof(float));
        printf("double size is: %lu\n", sizeof(double));
        printf("long double size is: %lu\n", sizeof(long double));
    }

    {
        float a = 3.1415926;
        double b = 5.55;
        long double c = 1.00001;
        printf("a = %f, b = %f, c = %Lf\n", a, b, c);
        printf("a = %.3f, a = %.4f, a = %.5f\n", a, a, a);
    }

    {
        printf("bool size is: %lu\n", sizeof(bool));
    }

    {
        bool a = true;
        bool b = false;
        printf("a (true) = %d, b (false) = %d\n", a, b);
    }

    {
        printf("char size is: %lu\n", sizeof(char));
    }

    {
        char a = 'A';
        char b = 'a';
        printf("a = %c (%d)\n", a, a);
        printf("b = %c (%d)\n", b, b);
        printf("a + 32 = %c (%d)\n", a + 32, a + 32);
    }

    {
        printf("char* size is: %lu\n", sizeof(char*));
        printf("char[128] size is: %lu\n", sizeof(char[128]));
    }

    {
        char* s1 = "Hello, World!";
        char* s2 = "Hello,\nC!";
        char* s3 = "Hello,\rRust!";
        char* s4 = "Hello,\bGo!";
        char* s5 = "Hello,\tMySQL!";
        char* s6 = "Hello,\fMongoDB!";
        char* s7 = "\" ' \' \\";
        printf("==> s1 = \n%s\n", s1);
        printf("==> s2 = \n%s\n", s2);
        printf("==> s3 = \n%s\n", s3);
        printf("==> s4 = \n%s\n", s4);
        printf("==> s5 = \n%s\n", s5);
        printf("==> s6 = \n%s\n", s6);
        printf("==> s7 = \n%s\n", s7);

        char buf[128] = {0};
        printf("==> buf = \n%s\n", buf);
        memset(buf, 0, 128);
        printf("==> buf = \n%s\n", buf);
    }

    {
        typedef char my_char;
        my_char c1 = 65;
        printf("c1 = %c %d\n", c1, c1);
    }

    return EXIT_SUCCESS;
}
