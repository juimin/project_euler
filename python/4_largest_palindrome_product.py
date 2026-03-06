def is_palindrome(s):
    n = len(s) // 2
    for x in range(n):
        front = s[x]
        back = s[len(s) - x - 1]
        if front != back:
            return False
    return True


tests = ["ada", "dds", "dd", "racecar"]
for x in tests:
    print(is_palindrome(x))


def find(a, b):
    max = 0
    for x in range(a, b, -1):
        for y in range(a, b, -1):
            n = x * y
            s = str(n)
            if is_palindrome(s):
                if n > max:
                    max = n
    print(max)


find(999, 99)
