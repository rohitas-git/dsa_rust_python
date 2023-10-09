
# Node of doubly linked list
class Node:
    def __init__(self, K):
        # data stored in node
        self.data = K 
        # reference to next connected node
        self.next = None 
        # reference to prev connected node
        self.prev = None 
        
def printList(head):
    if head == None:
        print(None) 
        return
    
    curr = head 
    while curr != None:
        print(curr.data, end=" ")
        curr = curr.next
    return 
     
        
n1 = Node(10)
n2 = Node(20)
n3 = Node(30)
h = n1

n1.prev = None 
n1.next = n2 

n2.prev = n1
n2.next = n3 

n3.prev = n2
n3.next = None

printList(h)
