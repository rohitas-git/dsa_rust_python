class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    list1 = createList([])
    printNthFromEnd_efficient(list1, 3)
    
    list2 = createList([1,2])
    printNthFromEnd_efficient(list2, 3)

    list3 = createList([1,99,3,4,5,6,7,8])
    printNthFromEnd_efficient(list3, 7)
    
    list4 = createList([1,2,3,99,5,6,7,8,9])
    printNthFromEnd_efficient(list4, 6)
    

# time - O(N) [one traversal], space- O(1)
# two pointers approach 
# - when first reaches None, second reaches the required node
def printNthFromEnd_efficient(head,n):
    if head == None:
        return
    
    first = head
    second = head 
    
    # second is at k+1th position from first
    # for second to be nth position from end, 
    # first has to be at None
    k = 0 
    while k != n:
        # if n > size of list
        if first == None:
            return 
        first = first.next 
        k+=1
    
    while first != None:
        first = first.next  
        second = second.next 
    print(second.key)
    return second    


    
# time O(N), space O(1)
# 1 traversal for counting the list, 
# 1 traversal for reaching the req. node
def printNthFromEnd_naive(head, n):
    if head == None:
        print(head)
        return 
    
    len = 1
    curr = head 
    while curr.next != None:
        curr = curr.next
        len+=1
    
    # position from start of list    
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