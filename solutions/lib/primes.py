import math

def seive(n):
    mem: list[bool] = [True for _ in range(n)]
    for i in range(2, int(math.sqrt(n)) + 1):
        if mem[i] is True:
            for j in range(i * i, n, i):
                mem[j] = False

    return [i + 2 for i, is_prime in enumerate(mem[2:]) if is_prime]


def find_widest_factor(n):
    # worst case n for primes ugh
    for x in range(2, n):
        if n % x == 0:
            return (x, int(n / x))
    return None, n


def prime_factor_set(t):
    """
    factor tree method
    """
    q = [t]
    primes = set()
    while q:
        n = q.pop(0)
        a, b = find_widest_factor(n)
        if a is None:
            # it's a prime
            primes.add(b)
        else:
            # it's not a prime
            q.append(a)
            q.append(b)
    return primes

def prime_factors(t):
    """
    factor tree method
    """
    q = [t]
    primes = []
    while q:
        n = q.pop(0)
        a, b = find_widest_factor(n)
        if a is None:
            # it's a prime
            primes.append(b)
        else:
            # it's not a prime
            q.append(a)
            q.append(b)
    return primes