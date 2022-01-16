t = 600851475143

primes = [2, 3, 5, 7]


def find_widest_factor(n):
    # worst case n for primes ugh
    for x in range(2, n):
        if n % x == 0:
            return (x, int(n / x))
    return None, n


# for p in primes:
#     print(find_widest_factor(p))


def largest_prime_factor(t):
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
    print(primes)


cases = [60, 282, 600851475143]
for c in cases:
    largest_prime_factor(c)
