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
lastSumOfGroup= float('inf')
answer=0
for i in range(0, len(sonarDataVector)-2):
    sonar1=sonarDataVector[i]
    sonar2=sonarDataVector[i+1]
    sonar3=sonarDataVector[i+2]
    sumOfGroup=sonar1+sonar2+sonar3
    if sumOfGroup>lastSumOfGroup:
        answer+=1
    lastSumOfGroup=sumOfGroup
print(answer)