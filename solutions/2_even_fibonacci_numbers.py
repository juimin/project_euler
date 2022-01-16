limit = 4000000

sum = 2

n1 = 1
n2 = 2

while n1 < limit:
    temp = n2
    n2 = n2 + n1
    n1 = temp
    if n2 % 2 == 0:
        sum += n2

print(f"Sum of even values in fibonacci seq under 4 million : {sum}")
