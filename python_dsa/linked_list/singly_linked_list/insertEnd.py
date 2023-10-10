class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    head = None
    head = insertEnd(head, 10)
    head = insertEnd(head, 20)
    head = insertEnd(head, 30)

    printList(head)
    
    head, last = None, None
    (head,last) = fasterInsertEnd(head, last, 1)
    (head,last) = fasterInsertEnd(head, last, 2)
    (head,last) = fasterInsertEnd(head, last, 3)
    
    printList(head)
    
    

# time - O(N), space O(1)
def insertEnd(head, key):
    to_add  = Node(key)
    
    if head == None:
        # if empty list return new node
        return to_add
    else:
        # reach last node: curr = last node
        curr = head 
        while curr.next != None:
            curr = curr.next 
        
        # add new node at end
        curr.next = to_add
        
        # return head of modified list
        return head


# time - O(1), space - O(1)
def fasterInsertEnd(head,last, key):
    to_add  = Node(key)
    
    if head == None:
        # if empty list return new node
        return (to_add,to_add)
    else:
        # reach last node: curr = last node
        last.next = to_add
        return (head, to_add)


def printList(head):
    traveller = head 
    print("-----LINKED LIST------")
    while traveller != None:    
        print(traveller.key)
        traveller = traveller.next 


main()