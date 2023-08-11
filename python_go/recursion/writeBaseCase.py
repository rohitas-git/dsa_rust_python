# Writing Base cases:
# - Draw the recursion tree of problem
# - See what its parameter are at its limit
# - Check the domain of parameter for which we input
# - Make sure that base case handle all limit cases, 
#       so that parameter can be any value in input domain
# - Make sure base case handles corner cases 


# given n >= 0 
def factorial(n):
    if n == 0:
        return 1
    return n*factorial(n-1)


# given n >= 0
def fibonacci(n):
    if n==0:
        return 0    
    if n==1:
        return 1
    return fibonacci(n-1) + fibonacci(n-2)

# Fibonacci's limit case F(1) and F(0)
# Need to handle base case for n=0 and n=1 
# if only handle n=0 and not n=1 then when F(1) is called, 
# F(-1) will also be called 
# But -1 is not within out input domain and thus, F(-1) is undefined -> ERROR
