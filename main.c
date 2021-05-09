
//  > cargo build --release
//  > gcc -O2 -Wl,--gc-sections main.c target/release/libdemo.a -o demo
//  > strip demo
//  > ./demo
//
//  hello world
//  42 12345678
//  enum test: 1

#include <stdio.h>

#include "bindings.h"

int main() {
    char mybuf[32];

    // Have the rust code write a string into our buffer.
    fill_buffer(mybuf, sizeof(mybuf));
    printf("%s\n", mybuf);


    TestStruct ts = { 0, 0 };

    // Have the rust code initialize a struct for us.
    fill_struct(&ts);

    printf("%u %lu\n", ts.x, ts.y);

    bool x = handle_enum(Two);
    printf("enum test: %u\n", x);
}
