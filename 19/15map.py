color = True
debug = False
program = [int(x) for x in open('15.input', 'r').read().strip().split(',')]
numbers = program[252:1032]
print(",".join(map(str, numbers)))
w = 39
h = 39
if color:
    red = '\x1b[31m'
    green = '\x1b[32m'
    reset = '\x1b[0m'
else:
    red = ''
    green = ''
    reset = ''
assert len(numbers) == w * (h + 1) // 2
chars = ['%s.' % reset if x < 37 else '%s#' % reset for x in numbers]
rawlines = [chars[i:i + 39] for i in range(0, len(numbers), 39)]
# the map data represents pairs of (odd y, even y) starting at (1,1)
doubled = [rawlines[row // 2] for row in range(0, h)]
# if x odd and y odd then free space
odds = [['%s.' % green if (x % 2 == 1) and (y % 2 == 1) else '?'
    for x in range(1, w+1)] for y in range(1, h+1)]
# else if x odd or y odd then do map lookup, else (both even) wall
evens = [['?' if (x % 2 == 1) or (y % 2 == 1) else '%s#' % red
    for x in range(1, w+1)] for y in range(1, h+1)]
oddevens = [[floor if floor != '?' else wall
    for floor, wall in zip(*pair)] for pair in zip(odds, evens)]
combined = [[oei if oei != '?' else rawi
    for oei, rawi in zip(*pair)] for pair in zip(oddevens, doubled)]

if debug:
    print('data:')
    print('\n'.join([''.join(line) for line in rawlines]))
    print('doubled:')
    print('\n'.join([''.join(line) for line in doubled]))
    print('odd:')
    print('\n'.join([''.join(line) for line in odds]))
    print('even:')
    print('\n'.join([''.join(line) for line in evens]))
    print('odd, even:')
    print('\n'.join([''.join(line) for line in oddevens]))
    print('combined:')

print('\n'.join([''.join(line) for line in combined]), reset)

if debug:
    print('found:')
    print(open('15.output.txt').read())
