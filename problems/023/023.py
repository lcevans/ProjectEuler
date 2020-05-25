def is_abundant(n):
    return sum(i for i in range(1,n) if n % i == 0) > n

abundant = [n for n in range(28124) if is_abundant(n)]

abundant_sums = {x + y for x in abundant for y in abundant}

print(sum(n for n in range(28124) if n not in abundant_sums))
