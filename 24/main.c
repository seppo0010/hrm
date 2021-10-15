#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include "../functions.h"

#define MEMORY_SIZE 10


int main() {
    const long input[] = {8,3,8,4,2,9};
    INIT();
    goto start;
out:
    COPYFROM(0);
    OUTBOX();
start:
    INBOX();
    COPYTO(0);
    INBOX();
    COPYTO(1);
sub:
    COPYFROM(0);
    SUB(1);
    IFNEG(out);
    COPYTO(0);
    goto sub;
}
