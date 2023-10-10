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
   
    h1 = deleteKth(h1,2)
    traverse(h1) 
    
    h2 = Node(10)
    h2.next = h2

    h3 = None
    
    
# time - theta(1)
def deleteKth(head, k):
    
    if head == None or head.next == head:
        return None
    
    if k == 1:
        return deleteHead(head)
    else:
        a = 1 
        curr = head
        while a != k-1:         # or for i in range(k-2):
            curr = curr.next 
            a+=1 
        curr.next = curr.next.next     
        return head 
    


# time - theta(1)
def deleteHead(head):
    
    if head == None or head.next == head:
        return None
    
    head.data = head.next.data # change data of head
    head.next = head.next.next # delete head.next
    
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