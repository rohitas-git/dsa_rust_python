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
    
    h1 = insertEnd_v2(h1, 5)
    traverse(h1) 
    
    h2 = Node(10)
    h2.next = h2

    h3 = None
    
    
# time - theta(1)
def insertEnd_v2(head, data):
    tmp = Node(data)
    curr = head 
    
    if head == None:
        tmp.next = tmp 
        return tmp
    
    tmp.next = curr.next 
    curr.next = tmp 
    tmp.data, curr.data = curr.data, tmp.data # swapping the data
    return tmp # tmp is the new head



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