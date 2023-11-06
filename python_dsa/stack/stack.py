
# Stack in Python
# - using List, 
# - using collections.deque [deque (double ended queue) impl using doubly linked list]
# - using queue.LIFOQUEUE [suitable for multi-threaded env]
# - using our own implementation

# ----------------------------------------------------------- 
# impl using List

# amortized time complexity for insertion and deletion at end of List - BigO(1)
# so better choose end as top to push/pop in stack

stack = [] 

stack.append(10) # push
stack.append(20)
stack.append(30)

print(stack.pop()) # pop

top = stack[-1] # peek
print(top)

print(len(stack)) #len  


#-------------------------------------------
# impl using collections.deque   

from collections import deque

stack = [] 

stack.append(10) # push
stack.append(20)
stack.append(30)

print(stack.pop()) # pop

top = stack[-1] # peek
print(top)

print(len(stack)) #len  

#--------------------------------------

# List impl
# + Cache friendly as its array implementation
# - Worst case O(N)

# Deque impl
# + worst case O(1) 
# - Not Cache friendly as its doubly linked list implementation

