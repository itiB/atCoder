if __name__ == "__main__":
	num = int(input())
	a = list(map(int, input().split()))

	sort = [0 for i in range(num)]

	for a_n in range(0, num):
		sort[a[a_n] - 1] = a_n + 1

	for sorted in sort:
		print(sorted, end = " ")

	print("")