
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
    h = deleteLast(h)
    printList(h)


# time - O(N)
# if maintaining tail variable, time - O(1) 
# but in that case, space has extra overhead maintaining tail variable in every operation
def deleteLast(head):
    if head == None or head.next == None:
        return None
    curr = head 
    while curr.next.next != None:
        curr = curr.next 
    curr.next = None
    return head
    


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