"""
The first known prime found to exceed one million digits was discovered in 1999, and is a Mersenne prime of the form 2^6972593−1; 
it contains exactly 2,098,960 digits. Subsequently other Mersenne primes, of the form 2p−1, have been found which contain more digits.

However, in 2004 there was found a massive non-Mersenne prime which contains 2,357,207 digits: 28433×2^7830457+1.

Find the last ten digits of this prime number.



"""
import math

# https://math.stackexchange.com/questions/186400/finding-the-last-digit-of-largest-mersenne-prime
# https://www.exploringbinary.com/patterns-in-the-last-digits-of-the-positive-powers-of-two/

# target = 7830457


# def calc_period(m):
#     return int(4 * math.pow(5, m - 1))


# periods = [calc_period(x) for x in range(1, 11)]

# print(periods)

# after_mods = [target % n for n in periods]

# print(after_mods)

# # print(math.pow(2, 17957))

# last_ten = 9700303872
# answer = last_ten * 28433 + 1
# print(answer)

# 275808739992577


def solution(digits: int, coefficient: int, expo: int, offset: int) -> int:
    """
    Calculates the last N digits following the equation

        Y = A*2^B + C

    This works because successive multiplications involving the trailing N
    digits truncates the integer we need to multiply for each exponent expansion

    In the case of 10 digits, we can compute 2 * X, truncate it to the last 10 digits,
    and then compute at least the last 10 digits with the next iteration. Further truncations
    prevent the value of n from exploding the memory / run time.
    """
    n = 1
    for _ in range(expo):
        # A bitshift multiplies by 2
        n <<= 1
        n %= pow(10, digits)

    n *= coefficient

    return n + offset


s = solution(10, 28433, 7830457, 1)

print(s)

