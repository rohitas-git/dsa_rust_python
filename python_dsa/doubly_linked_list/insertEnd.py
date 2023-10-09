
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
    h = insertEnd(h, 10)
    h = insertEnd(h, 20)
    h = insertEnd(h, 30)
    printList(h)


# time - O(N)
# if maintaining tail variable, time - O(1) 
# but in that case, space has extra overhead maintaining tail variable in every operation
def insertEnd(head, data):
    tmp = Node(data) 
    
    if head == None :
        return tmp 
    if head.next == None:
        head.next = tmp 
        tmp.prev = head 
        return head
    else:
        curr = head 
        while curr.next != None:
            curr = curr.next
        curr.next = tmp 
        tmp.prev = curr 
        return head


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