while True:
    i = input()
    if i == '':
        break
    hand = int(i)
    v1 = hand
    while True:
        hand = int(input())
        if hand == 0:
            hand = v1
            print(hand)
            break
        v2 = hand
        hand -= v1
        if hand < 0:
            hand = v2
            v1 = hand
        continue
