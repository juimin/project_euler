def is_triple(a, b, c):
    return (a * a) + (b * b) == (c * c)

for i in range(1, 1000):
    for j in range(i + 1, 1000):
        for k in range(j + 1, 1000):
            if i + j + k == 1000 and is_triple(i, j, k):
                print(f"{i} {j} {k}")
                print(f"product: {i * j * k}")
                exit()