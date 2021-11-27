def euclid(a: int, b: int)->int:
    a, b = min(a,b), max(a, b)
    while True:
        c = a%b
        if c == 0:
            return b
        a = b
        b = c

def euclid_count_steps(a: int, b: int)->int:
    a, b = min(a,b), max(a, b)
    count = 0
    while True:
        c = a%b
        if c == 0:
            return count
        a = b
        b = c
        count += 1

print(f"{euclid(1071, 462) = }, {euclid_count_steps(1071, 462) = }")
print(f"{euclid(44, 12) = }")
print(f"{euclid(92, 20) = }")
print(f"{euclid(92, 20) = }")
print(f"{euclid(1764, 7700) = }")



