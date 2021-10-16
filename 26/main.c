#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include "../functions.h"

#define MEMORY_SIZE 10

int main() {
    const long input[] = {3,5,3,1,10,2,1,2};
    INIT();
    memory[9] = 0;
    goto start;
out:
    COPYFROM(2);
    OUTBOX();
start:
    // 0: divisor
    // 1: to divide
    // 2: counter
    COPYFROM(9);
    COPYTO(2);
    INBOX();
    COPYTO(1);
    INBOX();
    COPYTO(0);
sub:
    COPYFROM(1);
    SUB(0);
    IFNEG(out);
    COPYTO(1);
    BUMPI(2);
    goto sub;
}
