#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MEMORY_SIZE 10

#define INBOX()\
    if (inputpos == sizeof(input) / sizeof(input[0])) return 0;\
    hand = input[inputpos++]

#define OUTBOX()\
    printf("%ld\n", hand)

#define COPYFROM(pos)\
    if (memory[pos] == INT_MAX) { printf("asking for position %d but was not initialized\n", pos); return 1; }\
    hand = memory[pos]

#define COPYTO(pos)\
    memory[pos] = hand

#define ADD(pos)\
    hand += memory[pos]

#define SUB(pos)\
    hand -= memory[pos]

#define BUMPI()\
    hand++

#define BUMPD()\
    hand--

#define IF0(label)\
    if (hand == 0) goto label

#define IFNEG(label)\
    if (hand < 0) goto label

#define DEBUG()\
    printf("hand: %ld\n", hand)

int main() {
    const long input[] = {8,3,8,4,2,9};
    long inputpos = 0;
    long memory[MEMORY_SIZE];
    for (long i = 0; i < MEMORY_SIZE; i++) memory[i] = INT_MAX;
    long hand = 0;
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
