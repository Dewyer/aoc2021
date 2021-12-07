import sys

# Read input
pilotCommandsRaw = sys.stdin.read()
position=[0,0]

#position[x,y] x:horizontal, y:depth
for row in pilotCommandsRaw.split('\n'):
    if row == '':
        continue
    
    movement=int(row.split(' ')[1])
    if row.startswith('forward'):
        position[0] += movement
    if row.startswith('up'):
        position[1] -= movement
    if row.startswith('down'):
        position[1] += movement
print(position[0]*position[1])