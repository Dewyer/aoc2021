import sys

# Read input
sensorDataRaw = sys.stdin.read()

rows = sensorDataRaw.split('\n')
bitCount = len(rows[0])

oneCounts = []
for ii in range(0, bitCount):
    oneCounts.append(0)

rowCount = 0

for row in rows:
    if row == '':
        continue
    rowCount += 1
    for digitIndex in range(0, len(row)):
        if row[digitIndex]=='1':
            oneCounts[digitIndex] += 1

gamma=''
epsilon=''

for count in oneCounts:
    if count >= rowCount/2:
        gamma += '1'
        epsilon += '0'
    else:
        gamma += '0'
        epsilon += '1'

answer = int(gamma,2)*int(epsilon,2)
print(answer)