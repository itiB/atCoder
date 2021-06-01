i = list(map(int, input().split()))

k = int(i[0])
x = int(i[1])

max = x + k - 1
min = x - k + 1

for num in range(min, max):
	print(num, end=" ")
print(max)