import sys

# Read input
pilotCommandsRaw = sys.stdin.read()
position=[0,0]

#position[x,y] x:horizontal, y:depth
aim=0

for row in pilotCommandsRaw.split('\n'):
    if row == '':
        continue
    
    movement=int(row.split(' ')[1])
    if row.startswith('forward'):
        position[0] += movement
        position[1] += aim*movement
    if row.startswith('up'):
        aim -= movement
    if row.startswith('down'):
        aim += movement
print(position[0]*position[1])