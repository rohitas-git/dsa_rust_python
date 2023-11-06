# Linked List impl of Stack
# - own impl of stack
# - deque in collections

# Better choice to push/pop at head of list instead of its end
# because at end, deletion/insert are linear

# underflow -> doing peek/pop operation when stack is empty 
# overflow -> doing push operation when stack is full

import math 

class Node:
    def __init__(self, d):
        self.data = d 
        self.next = None
        
        
class MyStack: 
    def __init__(self):
        self.head = None 
        self.size = 0
        
        
    def push(self, x):
        temp = Node(x)
        temp.next = self.head 
        self.head = temp 
        self.size += 1
    
    
    def len(self):
        return self.size
    
    
    def peek(self):
        if self.head is None: # handling underflow
            return math.inf  
        return self.head.data
    
    
    def pop(self):
        if self.head == None:  # handling underflow
            return math.inf  
        res = self.head.data 
        self.head = self.head.next 
        self.size -=1 
        return res 

    
# End of MyStack class

s = MyStack()
s.push(10)
s.push(20) 
s.push(30)

print(s.pop())
print(MyStack().peek())
print(s.len())
