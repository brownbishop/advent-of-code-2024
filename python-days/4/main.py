#!/usr/bin/python3
import sys

input_file = "input.txt"
if len(sys.argv) == 2 and sys.argv[1] == "-test":
    input_file = "example.txt"

data: dict[complex, str] = {}
with open(input_file, "r") as file:
    for i, line in enumerate(file.readlines()):
        for j, chr in enumerate(line.strip()):
            data[complex(i, j)] = chr

directions = [1, -1, 1j, -1j, 1+1j, 1-1j, -1+1j, -1-1j]
count = 0
for k, v in data.items():
    for dir in directions:
        path = v
        if path.startswith('X'):
            try:
                for i in range(1,4):
                    path += data[k + i * dir]
            except KeyError:
                pass
        if path == 'XMAS':
            count += 1



print("part one: ", count)

# part two
count = 0
for k, v in data.items():
    try:
        if v == 'A':
            diag1 = data[k - 1 - 1j] + 'A' + data[k + 1 + 1j]
            diag2 = data[k + 1 - 1j] + 'A' + data[k - 1 + 1j]
            expected = ["MAS", "SAM"]
            if diag1 in expected and diag2 in expected:
                count += 1
    except KeyError:
        pass

print("part two: ", count)




