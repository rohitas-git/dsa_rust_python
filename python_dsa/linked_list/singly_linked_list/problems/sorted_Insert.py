class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    head = Node(4)
    head.next = Node(6)
    head.next.next = Node(8)

    head = sortedInsert(head, 2)
    head = sortedInsert(head, 5)
    head = sortedInsert(head, 9)
    
    printList(head)
    

# time - O(N), space O(1)
# assume ascending order of values in list 
def sortedInsert(head, target):
    # case 0: empty list    : theta(1)
    # case 1: insert at beginning : theta(1)
    # case 2: insert in middle :O(N)
    # case 3: insert at end: theta(N)
    
    value = target 
    toAdd = Node(target)
    
    
    # case 0 
    if head == None:
        return toAdd 
    
    # case 1
    if head.key > value:
        toAdd.next = head
        return toAdd 
    
    # case 2 and case 3
    current = head 
    while current.next != None and current.next.key < value:
        current = current.next 
    toAdd.next = current.next
    current.next = toAdd
    return head 


def printList(head):
    traveller = head 
    while traveller != None:    
        print(traveller.key)
        traveller = traveller.next 


main()