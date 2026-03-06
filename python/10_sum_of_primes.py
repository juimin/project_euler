import math

def seive(n):
    mem: list[bool] = [True for _ in range(n)]
    for i in range(2, int(math.sqrt(n)) + 1):
        if mem[i] is True:
            for j in range(i * i, n, i):
                mem[j] = False

    return [i + 2 for i, is_prime in enumerate(mem[2:]) if is_prime]


print(len(seive(2000000)))