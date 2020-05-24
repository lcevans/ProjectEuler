NUM2WORD = {
    1: 'one',
    2: 'two',
    3: 'three',
    4: 'four',
    5: 'five',
    6: 'six',
    7: 'seven',
    8: 'eight',
    9: 'nine',
    10: 'ten',
    11: 'eleven',
    12: 'twelve',
    13: 'thirteen',
    14: 'fourteen',
    15: 'fifteen',
    16: 'sixteen',
    17: 'seventeen',
    18: 'eighteen',
    19: 'nineteen',
    20: 'twenty',
    30: 'thirty',
    40: 'forty',
    50: 'fifty',
    60: 'sixty',
    70: 'seventy',
    80: 'eighty',
    90: 'ninety',
}

for tens in range(2,10):
    for ones in range(1,10):
        NUM2WORD[10*tens + ones] = NUM2WORD[10 * tens] + " " + NUM2WORD[ones]

for hundreds in range(1,10):
    for tens in range(0,10):
        for ones in range(0,10):
            NUM2WORD[100*hundreds + 10*tens + ones] = NUM2WORD[hundreds]  \
                                                    + " hundred" \
                                                    + (" and " + NUM2WORD[10*tens + ones] if 10*tens + ones in NUM2WORD else "")

NUM2WORD[1000] = 'one thousand'


print(sum(len(word.replace(" ", "")) for word in NUM2WORD.values()))
