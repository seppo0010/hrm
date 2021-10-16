#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include "../functions.h"

#define MEMORY_SIZE 10

int main() {
    const long input[] = {1,2,3,1,3,2,2,1,3,2,3,1,3,1,2,3,2,1};
    INIT();
    memory[9] = 0;
    goto start;
start:
    INBOX();
    COPYTO(0);
    INBOX();
    COPYTO(1);
    INBOX();
    COPYTO(2);
    printf("input: %ld %ld %ld\n", memory[0], memory[1], memory[2]);
    SUB(1);
    IFNEG(if1gt2);
if2gt1:
    COPYFROM(2);
    SUB(0);
    IFNEG(if0gt2gt1);
    COPYFROM(1);
    SUB(0);
    IFNEG(if2gt0gt1);
    COPYFROM(0);
    OUTBOX();
    COPYFROM(1);
    OUTBOX();
    COPYFROM(2);
    OUTBOX();
    goto start;
if2gt0gt1:
    COPYFROM(1);
    OUTBOX();
    COPYFROM(0);
    OUTBOX();
    COPYFROM(2);
    OUTBOX();
    goto start;
if1gt2:
    COPYFROM(1);
    SUB(0);
    IFNEG(if0gt1gt2);
    COPYFROM(2);
    SUB(0);
    IFNEG(if1gt0gt2);
    COPYFROM(0);
    OUTBOX();
    COPYFROM(2);
    OUTBOX();
    COPYFROM(1);
    OUTBOX();
    goto start;
if1gt0gt2:
    COPYFROM(2);
    OUTBOX();
    COPYFROM(0);
    OUTBOX();
    COPYFROM(1);
    OUTBOX();
    goto start;
if0gt2gt1:
    COPYFROM(1);
    OUTBOX();
    COPYFROM(2);
    OUTBOX();
    COPYFROM(0);
    OUTBOX();
    goto start;
if0gt1gt2:
    COPYFROM(2);
    OUTBOX();
    COPYFROM(1);
    OUTBOX();
    COPYFROM(0);
    OUTBOX();
    goto start;
}
