#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include "../functions.h"

#define MEMORY_SIZE 6

int main() {
    const long input[] = {3,5,0};
    INIT();
    memory[5] = 0;
    goto start;
out:
    COPYFROM(0);
    OUTBOX();
start:
    INBOX();
    // 0: acc
    // 1: current
    COPYTO(0);
    IF0(out);
next:
    COPYTO(1);
    BUMPD(1);
    ADD(0);
    COPYTO(0);
    COPYFROM(1);
    IF0(out);
    goto next;
}
