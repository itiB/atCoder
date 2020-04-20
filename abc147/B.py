if __name__ == "__main__":
	i = list(map(int, input().split()))
	h = list(map(int, input().split()))

	n = i[0]
	k = i[1]

	count = 0

	for h_n in h:
		if h_n >= k:
			count = count + 1

	print(count)