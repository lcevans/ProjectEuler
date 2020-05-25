import itertools

def decimal_digits(n):
    # Decimal digits after decimal of 1 / n
    r = 10
    mod = 1
    while mod != 0:
        while r < n:
            yield 0
            r *= 10
        div, mod = divmod(r, n)
        yield div
        r = 10 * (r - n * div)


# HACK: treat 95% match as eventually equal
def eventually_equal(list1, list2):
    n = 0
    match = 0
    for a, b in zip(list1, list2):
        n += 1
        if a == b:
            match += 1
    return match / n > 0.95


def length_cycle(n):
    digits = list(itertools.islice(decimal_digits(n), 10000))
    if len(digits) < 10000: # No repeating
        return 0
    for k in range(1, len(digits)):
        if eventually_equal(digits, digits[k:]):
            return k


print(max((length_cycle(i), i) for i in range(1,1000))[1])
