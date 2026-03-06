def find_widest_factor(n):
    # worst case n for primes ugh
    for x in range(2, n):
        if n % x == 0:
            return (x, int(n / x))
    return None, n


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
    return primes


r = list(range(1, 11))

print(r)

r.reverse()
for x in r:
    prime_facs = largest_prime_factor(x)
    for p in prime_facs:
        print(p)
        if p in r:
            r.remove(p)

print(r)

y = 1
for x in r:
    y *= x

print(y)


def finder(n):
    r = list(range(1, n))
    x = 1
    while x:
        if all(x % a == 0 for a in r):
            return x
        x += 1


print(finder(21))
# lol brute force
