
class Node:
    def __init__(self, K):
        # data stored in node
        self.data = K 
        # reference to connected node
        self.next = None 

# You are given a circular linked list of size N. 
# You need to insert an element data just after the given position pos.
# The position of first element is 1. If the given position is greater than N, 
# then don't insert anything as it is not possible.

def insertKth(head, pos, data):
    tmp = Node(data)
    
    if head == None:
        return head
    if head.next == head:
        if pos == 1:
            head.next = tmp
            tmp.next = head
            return head
        else:
            return head
    if pos == 1:
        tmp.next = head.next
        head.next = tmp
        return head
    
    p = 2
    curr = head.next
    while p != pos and curr.next != head :
        curr = curr.next
        p+=1
    
    if curr.next == head and p!=pos:
        return head
    
    tmp.next = curr.next
    curr.next = tmp
    return head