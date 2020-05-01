from datetime import datetime

def fibonacci_rec(n):
    # 1 1 2 3 5 8 13
    if n < 3:
        return 1
    return fibonacci_rec(n - 1) + fibonacci_rec(n - 2)

def fibonacci_loop(n):
    # 1 1 2 3 5 8 13
    prev, current = 1, 1
    for _i in range(2, n):
        current, prev = current + prev, current
    return current

num = int(input())
start = datetime.now()
print(f"Recursive: {fibonacci_rec(num)} : {datetime.now() - start}")
start = datetime.now()
print(f"Loop: {fibonacci_loop(num)} : {datetime.now() - start}")

