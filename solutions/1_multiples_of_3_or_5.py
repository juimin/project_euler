target = 1000

sum = 0
for n in range(target):
    if n % 3 == 0 or n % 5 == 0:
        sum += n

print(f"Sum of natural numbers that are multiples of 3 or 5 is: {sum}")
