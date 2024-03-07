#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>

typedef struct _User {
    uint32_t id;
    char name[32];
    uint8_t age;    
} User;

User* create_user(uint32_t id, char* name, uint8_t age) {
    User* u = malloc(sizeof(User));
    memset(u, 0, sizeof(User));
    u->id = id;
    strcpy(u->name, name);
    u->age = age;
    return u;
}

void delete_user(User* u) {
    free(u);
}

char* show_user(User* u) {
    char* buf = malloc(64 * sizeof(char));
    memset(buf, 0, 64);
    sprintf(buf, "User{id: %d, name: %s, age: %d}", u->id, u->name, u->age);
    return buf;
}

typedef struct _Argb {
    uint8_t b;
    uint8_t g;
    uint8_t r;
    uint8_t a;
} Argb;

typedef union _Color {
    uint32_t color;
    Argb argb;
} Color;

int main(int argc, char** argv) {
    printf("*** *** Structure *** ***\n");

    {
        printf("--- --- --- ---\n");
        User u1 = {1000, "liu", 18};
        printf("User size: %lu\n", sizeof(User));
        char* u1_info = show_user(&u1);
        printf("u1 info: %s\n", u1_info);
    }

    {
        printf("--- --- --- ---\n");
        User* u1 = create_user(1001, "liu", 18);
        User* u2 = u1;
        User u3 = *u1;
        u2->age = 22;
        u3.age = 25;
        printf("u1: %p, info: %s\n", u1, show_user(u1));
        printf("u2: %p, info: %s\n", u2, show_user(u2));
        printf("u3: %p, info: %s\n", &u3, show_user(&u3));
        delete_user(u1);
    }

    {
        printf("--- --- --- ---\n");
        Color c;
	c.color = 0x11223344;
        printf("c: 0x%x\n", c.color);
        printf("a: 0x%x\n", c.argb.a);
        printf("r: 0x%x\n", c.argb.r);
        printf("g: 0x%x\n", c.argb.g);
        printf("b: 0x%x\n", c.argb.b);
    }

    return EXIT_SUCCESS;
}
