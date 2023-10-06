
class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
    

def printList(head):
    traveller = head 
    while traveller != None:    
        print(traveller.key)
        traveller = traveller.next 
    

# Creating memory fragments
n1 = Node(10)
n2 = Node(20)
n3 = Node(30)

# Connecting the fragments
n1.next = n2 
n2.next = n3 

# Head of List
head = n1

# Alternative impl of linked list
head = Node(101)
head.next = Node(202)
head.next.next = Node(303)

printList(head)


        
