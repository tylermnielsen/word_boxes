import sys

if len(sys.argv) < 3:
    print("Usage: python compare.py <file1> <file2>")
    exit(1)

a = sys.argv[1]
b = sys.argv[2]

a_boxes = []
b_boxes = []

with open(a, 'r') as f:
    box = ""
    for line in f:
        if line.strip() == "":
          a_boxes.append(box)
          box = "" 
        else:
          box += line 



with open(b, 'r') as f:
    box = ""
    for line in f:
        if line.strip() == "":
          b_boxes.append(box)
          box = "" 
        else:
          box += line 

print(sorted(a_boxes) == sorted(b_boxes))
