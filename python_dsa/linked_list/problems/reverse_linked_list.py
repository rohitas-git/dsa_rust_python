class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    list1 = createList([])
    reverseList_efficient(list1)
    
    list3 = createList([10])
    reverseList_efficient(list3)
    
    list4 = createList([1,2,3,4])
    reverseList_efficient(list4)
    
    list2 = createList([1,2,3])
    reverseList_efficient(list2)
    


# time O(N),  space O(1)
# Idea: Reverse the links btw nodes
def reverseList_efficient(head):
    # handle 3 pointers - prev, curr, next 
    prev = None
    curr = head
    while curr != None:
        next = curr.next 
        curr.next = prev  # reverse link of curr 
        prev = curr 
        curr = next 
    
    printList(prev)
    return prev 
        


# time O(N) [two traversals], space O(N)
# Idea: Replace the data from back to start
def reverseList_naive(head):
    # using an auxilary list 
    # and going to each node to replacing the data of those node
    # by using list impl of stack
    stack = []
    curr = head
    while curr != None:
        stack.append(curr.key)
        curr = curr.next 
        
    curr = head 
    while curr != None:
        curr.key = stack.pop()
        curr = curr.next 
    
    return head 

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