# Write recursive trees for fns and print the output on paper by yourself
# for n = 3

def f1(n):
    if n==0:
        return
    print(n)
    f1(n-1)
    print(n)
    

def f2(n):
    if n==0:
        return
    f2(n)
    print(n-1)
    f2(n)
    

def f3(n):
    if n<= 1:
        return 0
    else:
        return 1 + f3(n//2)
    
# f3 returns log2 of (N) - logarithm base 2 of N b 
print(f3(16))


def f4(n):
    if n==0:
        return
    f4(n//2)
    print(n%2)
    
# Prints binary representation of the number n
print(f4(-5))

