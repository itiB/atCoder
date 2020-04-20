i = list(map(int, input().split()))

num1 = int(i[0])
num2 = int(i[1])

max = num1 + num2

if max < num1 - num2:
	max = num1 - num2
if max < num1 * num2:
	max = num1 * num2

print(max)