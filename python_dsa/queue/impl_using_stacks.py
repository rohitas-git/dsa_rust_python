
# A queue can be implemented using two stacks. 
# Let queue to be implemented be q and stacks used to implement q be stack1 and stack2. 
# q can be implemented in two ways: 
# - Method 1 (By making enQueue operation costly):
# - Method 2 (By making deQueue operation costly):

# Method 2 is better than method 1
#   Method 1 moves all the elements twice in enQueue operation, 
#   while method 2 (in deQueue operation) moves the elements once 
#   and moves elements only if stack2 empty. 
#   So, the amortized complexity of the dequeue operation becomes Theta (1) 

# ------------Method 1--------------------------------
# Make sure that oldest entered element is always at the top of stack 1, 
# so that deQueue operation just pops from stack1.
# To put the element at top of stack1, stack2 is used.

# enQueue(q, x) - O(n): 
# While stack1 is not empty, push everything from stack1 to stack2.
# Push x to stack1 (assuming size of stacks is unlimited).
# Push everything back to stack1.

# deQueue(q) - O(1): 
# If stack1 is empty then error
# Pop an item from stack1 and return it

# Python3 program to implement Queue using 
# two stacks with costly enQueue() 

class Queue: 
	def __init__(self):
		self.s1 = []
		self.s2 = []

	def enQueue(self, x):
		# Move all elements from s1 to s2 
		while len(self.s1) != 0: 
			self.s2.append(self.s1[-1]) 
			self.s1.pop()

		# Push item into self.s1 
		self.s1.append(x) 

		# Push everything back to s1 
		while len(self.s2) != 0: 
			self.s1.append(self.s2[-1]) 
			self.s2.pop()

	# Dequeue an item from the queue 
	def deQueue(self):
        # if first stack is empty 
		if len(self.s1) == 0: 
			return -1;
	
		# Return top of self.s1 
		x = self.s1[-1] 
		self.s1.pop() 
		return x

# Driver code 
if __name__ == '__main__':
	q = Queue()
	q.enQueue(1) 
	q.enQueue(2) 
	q.enQueue(3) 

	print(q.deQueue())
	print(q.deQueue())
	print(q.deQueue())

#--------------------Method 2------------------------
# In en-queue operation, the new element is entered at the top of stack1. 
# In de-queue operation, if stack2 is empty then all the elements 
# are moved to stack2 and finally top of stack2 is returned. 

# enQueue(q,  x) - O(1)
# Push x to stack1 (assuming size of stacks is unlimited).

# deQueue(q) - O(N) in general & O(1) amortized time complexity.
#   1) If both stacks are empty then error.
#   2) If stack2 is empty
#        While stack1 is not empty, push everything from stack1 to stack2.
#   3) Pop the element from stack2 and return it.

# Python3 program to implement Queue using 
# two stacks with costly deQueue()

class Queue:
	def __init__(self):
		self.s1 = []
		self.s2 = []

	# EnQueue item to the queue
	def enQueue(self, x):
		self.s1.append(x)

	# DeQueue item from the queue
	def deQueue(self):

		# if both the stacks are empty
		if len(self.s1) == 0 and len(self.s2) == 0:
			return -1

		# if s2 is empty and s1 has elements
		elif len(self.s2) == 0 and len(self.s1) > 0:
			while len(self.s1):
				temp = self.s1.pop()
				self.s2.append(temp)
			return self.s2.pop()

		else:
			return self.s2.pop()

	# Driver code
if __name__ == '__main__':
	q = Queue()
	q.enQueue(1)
	q.enQueue(2)
	q.enQueue(3)

	print(q.deQueue())
	print(q.deQueue())
	print(q.deQueue())


#-------------------------Method 3----------------------------
# Queue can also be implemented using one user stack and one Function Call Stack.

# enQueue(x) - O(1)
#   1) Push x to stack1.

# deQueue - O(N):
#   1) If stack1 is empty then error.
#   2) If stack1 has only one element then return it.
#   3) Recursively pop everything from the stack1, store the popped item 
#     in a variable res,  push the res back to stack1 and return res

# Python3 program to implement Queue using 
# one stack and recursive call stack. 
class Queue:
	def __init__(self):
		self.s = []
		
	# Enqueue an item to the queue 
	def enQueue(self, data):
		self.s.append(data)
		
	# Dequeue an item from the queue 
	def deQueue(self):
		# Return if queue is empty
		if len(self.s) <= 0:
			return -1
		
		# pop an item from the stack
		x = self.s[len(self.s) - 1]
		self.s.pop()
		
		# if stack become empty
		# return the popped item
		if len(self.s) <= 0:
			return x
			
		# recursive call
		item = self.deQueue()
		
		# push popped item back to
		# the stack
		self.s.append(x)
		
		# return the result of 
		# deQueue() call
		return item
	
# Driver code 
if __name__ == '__main__':
	q = Queue()
	q.enQueue(1)
	q.enQueue(2)
	q.enQueue(3)
	
	print(q.deQueue())
	print(q.deQueue())
	print(q.deQueue()) 
	
