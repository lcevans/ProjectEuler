def fib():
    a = 1
    yield a
    b = 1
    yield b
    while True:
        a, b = b, a + b
        yield b

for i, n in enumerate(fib()): # 0 indexed
    if len(str(n)) >= 1000:
        print(i+1)
        break
