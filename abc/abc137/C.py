N = int(input())
s = [input() for i in range(N)]

dic = {"a": 0, "b": 0 , "c": 0, "d": 0 , "e": 0, "f": 0, "g": 0, "h": 0, "i": 0, "j": 0, "k": 0, "l": 0, "m": 0, "n": 0, "o": 0, "p": 0, "q": 0, "r": 0, "s": 0, "t": 0, "u": 0, "v": 0, "w": 0, "x": 0, "y": 0, "z": 0}
word_dict = {}

for num in range(N):
	word_dict[num] = {k: v for (k, v) in dic.items()}

	for moji in s[num]:
		word_dict[num][moji] = word_dict[num][moji] + 1
	# print(word_dict[num])

match_num = 0

for i in range(N - 1):
	for j in range(i + 1, N):
		# print(str(i) + ":" + str(j))
		if(all(word_dict[i][key] == word_dict[j][key] for key in dic)):
			match_num = match_num + 1

print(match_num)