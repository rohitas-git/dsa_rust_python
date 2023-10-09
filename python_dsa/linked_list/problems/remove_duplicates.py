class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    list1 = createList([])
    removeDuplicates(list1)
    
    list3 = createList([1,2,2,3,3,4,5,6,6,6])
    removeDuplicates(list3)
    
    list4 = createList([1,2,3,4,5,6])
    removeDuplicates(list4)
    


def removeDuplicates(head):
    curr = head
    
    while curr != None and curr.next != None:
        if curr.key == curr.next.key:
            curr.next = curr.next.next 
        else:
            curr = curr.next 
    
    printList(head)


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
    print('-------------------')

main()