# Node of single linked list
class Node:
    def __init__(self, K):
        # data stored in node
        self.data = K 
        # reference to connected node
        self.next = None 
     
     
     
     
def main(): 
    h1 = Node(10)
    h1.next = Node(20)
    h1.next.next = Node(30)
    h1.next.next.next = h1
   
    h1 = deleteLast(h1)
    traverse(h1) 
    
    h2 = Node(10)
    h2.next = h2

    h3 = None
    
    
# time - theta(1)
def deleteLast(head):
    current = head
    previous = None
    
    if head == None or head.next == head:
        return None
    
    while current.next != head:
       previous = current
       current = current.next 
        
    previous.next = current.next 
    head = previous.next
    
    return head 
    


def traverse(head):
    
    if head == None:
        print(None)
        return 
    print(head.data, end=" ")
    
    start= head.next     
    while start != head:
        print(start.data, end=" ")
        start = start.next 
    
        
    
main()