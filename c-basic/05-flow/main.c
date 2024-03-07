#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

int main(int argc, char** argv) {
    {
        uint32_t a = 55;
        int8_t b = -88;
        if (a > b) {
            printf("a > b : a=%u, b=%d\n", a, b);
        } else if (a < b) {
            printf("a < b : a=%u, b=%d\n", a, b);
        } else {
            printf("a == b : a=%u, b=%d\n", a, b);
        }
        int8_t c = a;
        uint32_t d = b;
        printf("c = %d, d = %u\n", c, d);
        printf("(int8_t)a = %d, (uint32_t)b = %u\n", (int8_t)a, (uint32_t)b);
    }

    {
        const uint8_t UP = 1;
        const uint8_t DOWN = 2;
        const uint8_t LEFT = 3;
        const uint8_t RIGHT = 4;
        uint8_t dir = 2;
        switch (dir) {
            case UP:
                printf("UP: dir=%u\n", dir);
                break;
            case DOWN:
                printf("DOWN: dir=%u\n", dir);
                break;
            // case LEFT, RIGHT: // error: expected ':' after 'case'
            //     printf("LEFT or RIGHT: dir=%u\n", dir);
            //     break;
            case LEFT:
                printf("LEFT: dir=%u\n", dir);
                break;
            case RIGHT:
                printf("RIGHT: dir=%u\n", dir);
                break;
            default:
                printf("INVALID: dir=%u\n", dir);
                break;
        }
        switch (dir) {
            case UP:
                printf("UP: dir=%u\n", dir);
            case DOWN:
                printf("DOWN: dir=%u\n", dir);
            case LEFT:
                printf("LEFT: dir=%u\n", dir);
            case RIGHT:
                printf("RIGHT: dir=%u\n", dir);
            default:
                printf("INVALID: dir=%u\n", dir);
        }
    }

    {
        int8_t num = 0;
        START:
        num += 1;
        printf("num = %d\n", num);
        if (num < 10) {
            goto START; 
        }

        printf("--- --- --- ---\n");
        num = 0;
        START_2:
        num = num + 1;
        printf("num = %d\n", num);
        if (num > 10) {
            goto END_2;
        } else {
            goto START_2;
        }
        END_2:
        printf("--- --- --- ---\n");
    }

    {
        int8_t num = 10;
        int8_t sum = 0;
        for (int i = 1; i <= num; i++) {
            sum += i;
            printf("sum = %d\n", sum);
        }
        
        for (int i = 1; i <= 9; i++) {
            for (int j = 1; j <= i; j++) {
                printf("%d x %d = %d\t", j, i, i*j);
            }
            printf("\n");
        }

        for (int i=1; i<10; i++) {
            for (int j=1; j<=i; j++) {
                if (j == 3) {
                    continue;
                } else if (j == 5) {
                    break;
                } else if (i == 8 && j == 4) {
                    goto END;
                }
                printf("%d x %d = %d\t", j, i, i*j);
            }
            printf("\n");
        }
        END:
        printf("\n");
    }

    {
        int8_t m = 10;
        while (m > 0) {
            printf("m = %d\n", m);
            m = m - 1;
        }
        printf("--- --- --- ---\n");
        m = 10;
        do {
            printf("m = %d\n", m);
            m = m - 1;
        } while (m > 0);
        printf("--- --- --- ---\n");
        m = 0;
        while (m > 0) {
            printf("m = %d\n", m);
            m = m - 1;
        }
        printf("--- --- --- ---\n");
        m = 0;
        do {
            printf("m = %d\n", m);
            m = m -1;
        } while (m > 0);
    }

}
