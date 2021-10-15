last1, last2 = 1,1
print(1)
print(1)
z = int(input())
newval = last1+last2
while True:
    newval = last1 + last2
    if newval > z:
        break
    print(newval)
    last1 = last2
    last2 = newval
