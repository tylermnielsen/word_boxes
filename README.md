# WordBoxes
A program written in Rust to find WordBoxes, solid squares of letters that make valid words horizontally and vertically (playable in a game like bananagrams or scrabble). 

Utilizes a trie with prefix matching to more effectively traverse the combination set to find valid word boxes. 

### Use
```
./word_boxes.exe [input file] [width] [height] [output file]
```

### Example boxes 
#### 3x3 
```
was
are
see
```

#### 3x4
```
gas
are 
men 
eat
```

#### 4x4 
```
with
idea
sell
half
```

#### 6x4
```
corner
anyone
stands
honest
```

#### 6x6
```
patrol
ashore
thomas
romans
orange
lessen
```

#### 7x3
```
federal
average
related
```

Usable word banks are in [data](/data/) (a smaller dataset of common words, scrabble dictionary, and full english word list) 

Output files are in [output](/output/) with run stats in [runs.csv](/runs.csv)


### Most Common Word Source
https://github.com/david47k/top-english-wordlists (with words containing non alphabetic characters filtered out)
