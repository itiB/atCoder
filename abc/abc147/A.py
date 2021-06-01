if __name__ == "__main__":
	num = int(input())

	if num % 2 == 0:
		print("0.5")
	else:
		print(str((num // 2 + 1) / num))