from functools import reduce
N = reduce(lambda a,b: a*b, range(1,101))
print(sum(int(c) for c in str(N)))
