
#! Recursion Introduction

# The process in which a function calls itself directly or indirectly is called recursion 
# and the corresponding function is called a recursive function. 
# Using a recursive algorithm, certain problems can be solved quite easily

# A recursive function solves a particular problem by calling a copy of itself 
# and solving smaller subproblems of the original problems. 

# Many more recursive calls can be generated as and when required. 
# It is essential to know that we should provide a certain case in order to terminate this recursion process. 
# So we can say that every time the function calls itself with a simpler version of the original problem.

# A task that can be defined with its similar subtask, recursion is one of the best solutions for it.

#! Properties of Recursion:
# - Performing the same operations multiple times with different inputs.
# - In every step, we try smaller inputs to make the problem smaller.
# - Base condition is needed to stop the recursion otherwise infinite loop will occur.

#! How are recursive functions stored in memory?
# Recursion uses more memory, 
# because the recursive function adds to the stack with each recursive call, 
# and keeps the values there until the call is finished. 
# The recursive function uses LIFO (LAST IN FIRST OUT) Structure just like the stack data structure.

#! How a particular problem is solved using recursion? 
# The idea is to represent a problem in terms of one or more smaller problems, 
# and add one or more base conditions that stop the recursion.

#! How memory is allocated to different function calls in recursion? 
# When any function is called from main(), the memory is allocated to it on the stack. 
# A recursive function calls itself, the memory for a called function is allocated on top 
# of memory allocated to the calling function and a different copy of local variables is created 
# for each function call. When the base case is reached, the function returns its value 
# to the function by whom it is called and memory is de-allocated and the process continues

#! Recursion VS Iteration

# SR No.	Recursion	                                            Iteration
# 1)	Terminates when the base case becomes true.	                Terminates when the condition becomes false.
# 2)	Used with functions.	                                    Used with loops.
# 3)	Every recursive call needs extra space in the stack memory.	Every iteration does not require any extra space.
# 4)	Smaller code size.	                                        Larger code size.

#! Summary
# There are two types of cases in recursion i.e. recursive case and a base case.
# The base case is used to terminate the recursive function when the case turns out to be true.
# Each recursive call makes a new copy of that method in the stack memory.
# Infinite recursion may lead to running out of stack memory.
# Examples of Recursive algorithms: Merge Sort, Quick Sort, Tower of Hanoi, Fibonacci Series, Factorial Problem, etc.

# def fun(...):
    
    # base cases
    
    # recursive call with atleast one change to a parameter, so that call approaches zero


# Direct recursion
def fun1():
    fun1()

# Indirection recursion 
def fun2():
    fun3()
    
def fun3():
    fun2()



# gives: RecursionError: maximum recursion depth exceeded while calling a Python object
def callGfg():
    print("gfg")
    callGfg()
    
# callGfg()


def callGfg(n):
    if n<=0: 
        return
    print("gfg")
    callGfg(n-1)

callGfg(5)