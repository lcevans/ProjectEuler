
# is_prime function from stack overflow
from math import sqrt
from itertools import count, islice
def is_prime(n):
    return n > 1 and all(n % i for i in islice(count(2), int(sqrt(n)-1)))


def num_primes(a, b):
    n = 0
    count = 0
    while is_prime(n*n + a*n + b):
        count += 1
        n += 1
    return count

best = max((num_primes(a, b), a, b) for a in range(-999, 1000) for b in range(-999, 1000))

print(best[1] * best[2])
