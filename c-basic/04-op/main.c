#include <stdlib.h>
#include <stdio.h>
#include <math.h>

#define MALE 1
#define FEMALE 2

int main(int argc, char** argv) {
    {
        int32_t a = 54 + 8 / 2;
        int32_t b = (54 + 8) / 2;
        int32_t c = 5 / 2;
        int32_t d = 5 % 2;
        printf("a = %d, b = %d, c = %d, d = %d\n", a, b, c, d);
    }

    {
        double a = 2 * 2 * M_PI;
        double b = sin(M_PI/2);
        printf("a = %f, b = %f\n", a, b);
    }

    {
        // uint8_t score = 66;
        // uint8_t score = 101;
        uint8_t score = 56;
        if (score >= 60 && score <= 100) {
            printf("Ok!\n");
        } else if (score <= 100) {
            printf("Bad!\n");
        } else {
            printf("Invalid!\n");
        }
    }

    {
        uint8_t score = 55;
        if (score < 60 || score > 100) {
            printf("Bad or Invalid (score = %d)\n", score);
        } else {
            printf("Ok (score = %d)\n", score);
        }
    }

    {
        uint8_t sex = MALE;
        if (sex == FEMALE) {
            printf("sex is FEMALE\n");
        } else {
            printf("sex is MALE\n");
        }
        if (sex != FEMALE) {
            printf("sex is MALE\n");
        } else {
            printf("sex is FEMALE\n");
        }
    }

    {
        uint8_t a = 0b01;
        uint8_t b = 0b10;
        uint8_t c = a & b;
        uint8_t d = a | b;
        uint8_t e = a ^ b;
        uint8_t f = ~a;
        uint8_t g = b >> 1;
        uint8_t h = b << 3;
        printf("a = %d, b = %d, c = %d, d = %d, e = %d, f = %d, g = %d, h = %d\n", a, b, c, d, e, f, g, h);
    }

    {
        uint32_t color_argb = 0xABCDEF08;
        uint8_t a = (color_argb & 0xFF000000) >> 24;
        uint8_t r = (color_argb & 0x00FF0000) >> 16;
        uint8_t g = (color_argb & 0x0000FF00) >> 8;
        uint8_t b = (color_argb & 0x000000FF) >> 0;
        printf("color_argb = 0x%X, a=0x%X, r=0x%X, g=0x%X, b=0x%X\n", color_argb, a, r, g, b);
    }

    {
        char a = 'x';
        char b = 'Y';
        char c = 'z';
        putchar(a);
        putchar(b);
        putchar(c);
        putchar('\n');
        
        char* s1 = "Hello, World!";
        puts(s1);
        char s2[] = "Hello, C!";
        puts(s2);
    }

    {
        int32_t a = 66;
        int32_t b = -88;
        printf("a = %u, b = %u\n", a, b); // a = 66, b = 4294967208
        printf("a = %d, b = %d\n", a, b); // a = 66, b = -88
        printf("a = %4d\n", a);
        printf("a = %04d\n", a);
    }

    {
        printf("Please input a character: \n");
        char a = getchar();
        printf("Your input is %c\n", a);
        
        printf("Please input a number: \n");
        int32_t b = 0;
        scanf("%d", &b);
        printf("Your input is %d\n", b);
        
        printf("Please input a string: \n");
        char buf[100] = {0};
        memset(buf, 0, 100);
        scanf("%s", buf);
        printf("Your input is: \"%s\"\n", buf);
    }

    return EXIT_SUCCESS;
}
