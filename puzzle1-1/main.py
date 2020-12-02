import itertools
import sys

with open('input', 'r') as lines:
    pairs = itertools.combinations(map(int, lines), 2)
    for first, second in pairs:
        if first + second == 2020:
            print(f'{first} × {second} = {first * second}')
            sys.exit(0)
print('Not found')
sys.exit(1)
