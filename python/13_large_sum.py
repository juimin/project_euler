DIGITS = 50

with open("../resources/8_largest_product.txt", "r") as f:
    lines = f.readlines()

cache = {}
for i, line in enumerate(lines):
    cache[i] = line

sum = []
sum_at_place = 0
for digit in range(DIGITS - 1, 0, -1):
    for v in cache.values():
        sum_at_place += int(v[digit])

    last_digit = sum_at_place % 10

    sum.insert(0, last_digit)

    sum_at_place = sum_at_place // 10

while sum_at_place > 0:
    last_digit = sum_at_place % 10

    sum.insert(0, last_digit)

    sum_at_place = sum_at_place // 10

print(sum)

print("".join(str(i) for i in sum[:10]))