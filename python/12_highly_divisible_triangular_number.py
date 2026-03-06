from collections import defaultdict

from lib.primes import prime_factors

def get_triangular_number(n):
    return sum(range(1, n + 1))

def calculate_divisors(n):
    # trick 529
    pfs = prime_factors(n)
    counts = defaultdict(int)
    for p in pfs:
        counts[p] += 1

    divisors = 1
    for _, v in counts.items():
        divisors *= (v + 1)
    
    return divisors

max_divisors = 0
i = 0
while max_divisors <= 500:
    i += 1
    triangle = get_triangular_number(i)
    divisors = calculate_divisors(triangle)
    if divisors > max_divisors:
        max_divisors = divisors

print(f"The {i}th triangular number {get_triangular_number(i)} has {calculate_divisors(get_triangular_number(i))} divisors")