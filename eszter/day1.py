import sys

# Read input
sonarDataRaw = sys.stdin.read()

# process
def convertRawSonarToVector(rawSonar):
    vector = []

    for row in rawSonar.split('\n'):
        if row != '':
            vector.append(int(row))

    return vector

sonarDataVector = convertRawSonarToVector(sonarDataRaw)
answer=0
for i in range(0, len(sonarDataVector) - 1):
    sonar1 = sonarDataVector[i]
    sonar2 = sonarDataVector[i + 1]

    if sonar2>sonar1:
        answer += 1

print(answer)