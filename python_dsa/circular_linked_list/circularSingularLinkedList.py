
# * Circular Linked List
# - last Node is connected to first Node
# - can be singly or doubly circular linked list

# Node of single linked list
class Node:
    def __init__(self, K):
        # data stored in node
        self.data = K 
        # reference to connected node
        self.next = None 
     
     
# Creating memory fragments   
n1 = Node(10)
n2 = Node(20)
n3 = Node(30)

# Connecting the fragments
n1.next = n2 
n2.next = n3 
n3.next = n1 

# head of list
head = n1 

# --------------Alternative impl of linked list-------------------
head = Node(101)
head.next = Node(202)
head.next.next = Node(303)
head.next.next.next = head
