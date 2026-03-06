from functools import reduce

lines: list[str]

with open("../resources/8_largest_product.txt", "r") as f:
    lines = f.readlines()

full_string = "".join(l.strip() for l in lines)

window_size = 13

largest_product = 0
for i in range(0, len(full_string) - 14):
    digits = [int(digit) for digit in full_string[i:i+13]]
    product = reduce(lambda x, y: x*y, digits)
    if product > largest_product:
        largest_product = product

print(largest_product)