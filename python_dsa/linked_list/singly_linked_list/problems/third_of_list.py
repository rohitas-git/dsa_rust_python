class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    list1 = createList([])
    third(list1)
    
    list2 = createList([1,2])
    third(list2)
    
    list3 = createList([1,2,99,4,5,6,7,8])
    third(list3)
    
    list4 = createList([1,2,3,99,5,6,7,8,9])
    third(list4)

    

# time - theta(N) [one traversal], space - O(1) 
def third(head):
    if head == None:
        print(head)
        return 
    
    slow = head 
    fast = head 
    
    while fast != None and fast.next != None and fast.next.next != None:
        slow = slow.next 
        fast = fast.next.next.next 
    
    print(slow.key)


def createList(arr):
    head, last = None, None    
    for item in arr:
        (head,last) = fasterInsertEnd(head, last, item)
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
    while traveller != None:    
        print(traveller.key)
        traveller = traveller.next 


main()