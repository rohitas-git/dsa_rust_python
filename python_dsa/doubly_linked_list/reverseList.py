
# Node of doubly linked list
class Node:
    def __init__(self, K):
        # data stored in node
        self.data = K 
        # reference to next connected node
        self.next = None 
        # reference to prev connected node
        self.prev = None 
        

def main():
    h = None 
    h = insertBegin(h, 10)
    h = insertBegin(h, 20)
    h = insertBegin(h, 30)
    h = reverse(h)
    printList(h)


# time - Theta(N)
def reverse(head):
    if head == None:
        return None
    if head.next == None:
        return head 
    
    curr = head 
    prev= None 
    while curr != None:
        prev = curr 
        curr.next, curr.prev = curr.prev, curr.next
        curr = curr.prev
    return prev
    

# time - O(1)
def insertBegin(head, data):
    tmp = Node(data) 
    
    if head != None:
        head.prev = tmp 

    tmp.next = head 
    return tmp 


def printList(head):
    if head == None:
        print(None) 
        return
    
    curr = head 
    while curr != None:
        print(curr.data, end=" ")
        curr = curr.next
    return 


main()