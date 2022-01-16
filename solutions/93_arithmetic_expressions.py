"""
number of ways to order the operators

    4 * 4 * 4 = 64

number of ways to arrange the numbers

    P(n, r) = n! / (n - r)!

    n = number of things (4)
    r = pick count (4)

    P(4, 1) = 4! / (4 - 4)! 4 * 3 * 2 * 1 = 24   

number of ways to arrange parens


    

"""
import math

print(math.perm(4, 4))

