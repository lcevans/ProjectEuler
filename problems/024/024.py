import itertools
permutations = list(itertools.permutations(['0','1','2','3','4','5','6','7','8','9']))
print(''.join(permutations[1000000 - 1]))
