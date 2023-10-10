class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    head = Node(101)
    head.next = Node(202)
    head.next.next = Node(303)

    insertPosition(head, 10, 2)
    insertPosition(head, 20, 3)
    
    printList(head)
    

# time - theta( min(position, N)), space O(1)
def insertPosition(head, target, position):
    tmp = Node(target)
    
    if position == 1:
        tmp.next = head
        return tmp
    
    current = head
    pos = 1
    
    for i in range(position-2):
        current = current.next 
        if current == None:
            return head

    # insert at correct position node    
    tmp.next = current.next
    current.next = tmp
    return head


def printList(head):
    traveller = head 
    while traveller != None:    
        print(traveller.key)
        traveller = traveller.next 


main()