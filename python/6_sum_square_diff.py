t = 10


def sum_squared(n):
    s = sum(range(n + 1))
    return s * s


def sum_of_squares(n):
    x = 0
    for a in range(n + 1):
        x += a * a
    return x


def compute(n):
    return sum_squared(n) - sum_of_squares(n)


for x in [10, 100]:
    print(compute(x))
