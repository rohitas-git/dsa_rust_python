class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    list1 = createList([])
    nth(list1, 3)
    
    list2 = createList([1,2])
    nth(list2, 3)

    list3 = createList([1,2,99,4,5,6,7,8])
    nth(list3, 8)
    
    list4 = createList([1,2,3,99,5,6,7,8,9])
    nth(list4, 6)
    
    
    
def nth(head, n):
    if head == None:
        print(head)
        return 
    
    len = 1
    curr = head 
    while curr.next != None:
        curr = curr.next
        len+=1
        
    pos = (len-n+1)
    
    if pos < 1:
        print(None)
        return 
    
    p = 1
    curr = head 

    for i in range(1, pos):
        curr = curr.next
    print(curr.key)
    return 
    


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