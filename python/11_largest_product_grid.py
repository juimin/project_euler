with open("../resources/11_largest_product_grid.txt", "r") as f:
    lines = f.readlines()


matrix = []
for l in lines:
    matrix.append([int(i) for i in l.split(" ")])

print(matrix)

largest_product = 0

chain_length = 4

moves = [(1,1), (0,1), (1,0)]

for move in moves:
    for row_start in range(len(matrix)):
        for col_start in range(len(matrix[0])):
            try:
                product = 1
                nums = []
                for c in range(chain_length):
                    val = matrix[row_start + (c * move[0])][col_start + (c * move[1])]
                    product *= val
                    nums.append(val)
                if  product > largest_product:
                    largest_product = product
                print(nums)
            except IndexError:
                # don't do invalid ones
                pass

print(f"{largest_product=}")