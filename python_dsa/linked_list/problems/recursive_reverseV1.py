# Recursive Reverse A Linked List 

class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    list1 = createList([])
    printList(reverseList_v1(list1))
    
    list3 = createList([10])
    printList(reverseList_v1(list3))
    
    list4 = createList([1,2,3,4])
    printList(reverseList_v1(list4))
    
    list2 = createList([1,2,3])
    printList(reverseList_v1(list2))
    


# time Theta(N),  space Theta(N)
# Idea: 
# To reach the last node of the linked list using recursion 
# then start reversing the linked list. (going right to left)
def reverseList_v1(head):
    # base case
    if head == None:
        return head
    if head.next == None:
        return head
    
    # reverse rest of list except head
    rest_head = reverseList_v1(head.next)
    
    # reverse head and rest-tail
    rest_tail = head.next 
    rest_tail.next = head 
    head.next = None
    return rest_head     

    

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