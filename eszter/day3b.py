import sys

# Read input
sensorDataRaw = sys.stdin.read()

rows = sensorDataRaw.split('\n')
bitCount = len(rows[0])

def getCommonalitiBits(rowsActual):
    oneCounts = []
    for ii in range(0, bitCount):
        oneCounts.append(0)

    rowCount = 0
    filterdRows = []

    for row in rowsActual:
        if row == '':
            continue
        rowCount += 1
        filterdRows.append(row)

        for digitIndex in range(0, len(row)):
            if row[digitIndex]=='1':
                oneCounts[digitIndex] += 1

    o2Filter=''
    coFilter=''

    for count in oneCounts:
        if rowCount % 2 == 0 and count == rowCount / 2:
            # equal
            o2Filter += '1'
            coFilter += '0'
        if count >= rowCount/2:
            o2Filter += '1'
            coFilter += '0'
        else:
            o2Filter += '0'
            coFilter += '1'

    return [o2Filter, coFilter]

# got filters

def filterValuesForCriteria(useO2):
    currentArr = rows

    for toIndex in range(0, bitCount):
        nextArr = []
        commons = getCommonalitiBits(currentArr)
        currentFilter = commons[0] if useO2 else commons[1]

        for el in currentArr:
            if el == '':
                continue

            if el[toIndex] == currentFilter[toIndex]:
                nextArr.append(el)

        currentArr = nextArr
        if len(currentArr) == 1:
            return currentArr[0]

    return currentArr[0]

oxi = filterValuesForCriteria(True)
cotwo = filterValuesForCriteria(False)

oxiDec = int(oxi, 2)
coDec = int(cotwo, 2)

print(oxiDec * coDec)