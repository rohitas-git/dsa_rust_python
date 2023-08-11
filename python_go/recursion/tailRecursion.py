# Tail Recursive Function
# A recursive fn is called tail recursive 
# if the fn doesn't do anything after the last recursive call

# At last, caller only calls recursive fn and then it ends

# Why need to study it?
# The tail-recursive functions are considered better than non-tail recursive functions as tail-recursion can be optimized by the compiler. 
# The idea used by compilers to optimize tail-recursive functions is simple, 
# since the recursive call is the last statement, there is nothing left to do 
# in the current function, so saving the current function’s stack frame is of no use.

# Tail recursive fns are typically optimized by modern compilers 
# This optimization is called tail call elimination. 
# It elimination the recursive call stack as it changes the last recursive call to a looped fn call with n, n-1, n-2...

# Python doesn't do this. C/C++, Java do this

# Examples:
# - QuickSort 
# - PostOrder Tree traversal


def f1(n):
    if n==0:
        return
    print(n,end="")
    
    f1(n-1)
    
# Easy to change tail recursive fn to iterative fn
# - Replace if with while 
# - Change values of parameters at the end of the loop 
def f1_v2(n):
    while n!=0:
        print(n)
        n-=1
    
    
# f1 - Tail Recursive fn
# f2, f3 - Non tail recursive 


def f2(n):
    if n==0:
        return

    f1(n-1) 
    print(n,end="")
    
    
# Can a non-tail recursive function be written as tail-recursive to optimize it?
# ->It is not possible for every non tail recursive function to be written as tail recursive.
# ->Consider the following function to calculate factorial of N. 
#       Although it looks like Tail Recursive at first look, it is a non-tail-recursive function. 
#       If we take a closer look, we can see that the value returned by fact(N-1) is used in fact(N), 
#       so the call to fact(N-1) is not the last thing done by fact(N).
def f3(n):
    if n==0:
        return 1

    return n * f3(n-1)

# The above function can be written as a Tail Recursive function. 
# The idea is to use one more argument and accumulate the factorial value in the second argument.
# When N reaches 0, return the accumulated value.
def factTR(N, a):
    if (N == 0):  
        return a 
  
    return factTR(N-1, N*a)
