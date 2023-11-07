
# Design a Data Structure SpecialStack that supports all the stack operations like push(), pop(), isEmpty(), isFull() 
# and an additional operation getMin() which should return minimum element from the SpecialStack. 
# All these operations of SpecialStack must have a time and space complexity of O(1). 

# Approach: To solve the problem follow the below idea:

# We define a variable minEle that stores the current minimum element in the stack. 
# Now the interesting part is, how to handle the case when the minimum element is removed. 
# To handle this, we push “2x – minEle” into the stack instead of x 
# so that the previous minimum element can be retrieved using the current minEle 
# and its value stored in the stack

class Node:
    def __init__(self, d):
        self.data = d 
        self.next = None
        
    # This method returns the string representation of the object.
    def __str__(self):
        return "Node({})".format(self.value)
    
    # __repr__ is same as __str__
    __repr__ = __str__


class SpecialStack:
    def __init__(self):
        self.top = None 
        self.count = 0
        self.minimum = 0
     
    # This method returns the string representation of the object (stack).
    def __str__(self):
        temp = self.top
        out = []
        while temp:
            out.append(str(temp.value))
            temp = temp.next
        out = '\n'.join(out)
        return ('Top {} \n\nStack :\n{}'.format(self.top, out))
   
    # __repr__ is same as __str__
    __repr__ = __str__
    
    
    def isEmpty(self):
        # If top equals to None then stack is empty
        if self.top == None:
            return True
        else:
            # If top not equal to None then stack is empty
            return False
    
    # This method returns length of stack
    def __len__(self):
        self.count = 0
        tempNode = self.top
        while tempNode:
            tempNode = tempNode.next
            self.count += 1
        return self.count
        
        
    # This method returns top of stack
    def peek(self):
        if self.top is None:
            print("Stack is empty")
        else:
            if self.top.data < self.minimum:
                print("Top Most Element is: {}" .format(self.minimum))
            else:
                print("Top Most Element is: {}" .format(self.top.data))
    
    def push(self, data):
        if self.top is None :
            self.top = Node(data)
            self.count +=1
            self.minimum = data 
        
        elif data >= self.minimum:
            n = Node(data)
            n.next = self.top 
            self.top = n  
            self.count +=1
        else:
            tmp = 2*data - self.minimum
            new_node = Node(tmp)
            new_node.next = self.top 
            self.top = new_node  
            self.count +=1
            self.minimum = data 
            
        print("Number Inserted: {}" .format(data))

        
    def pop(self):
        if self.top is None:
            print("Stack is empty")
            return 
        
        popped = self.top.data 
        self.top = self.top.next 
        self.count -= 1
        
        if popped < self.minimum:
            print("Top Most Element Removed : {}" .format(self.minimum))
            self.minimum = 2*self.minimum - popped
        else:
            print("Top Most Element Removed : {}" .format(popped))
        
        
    def getMin(self):
        if self.top is None:
            return "Stack is empty"
        else:
            print("Minimum Element in the stack is: {}" .format(self.minimum))
        
    
# Driver program to test above class
if __name__ == '__main__':

    stack = SpecialStack()

    # Function calls
    stack.push(3)
    stack.push(5)
    stack.getMin()
    stack.push(2)
    stack.push(1)
    stack.getMin()
    stack.pop()
    stack.getMin()
    stack.pop()
    stack.peek()