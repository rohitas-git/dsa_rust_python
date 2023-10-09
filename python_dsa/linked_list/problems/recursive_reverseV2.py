# Recursive Reverse A Linked List 

class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    list1 = createList([])
    printList(reverseList_v2(list1))
    
    list3 = createList([10])
    printList(reverseList_v2(list3))
    
    list4 = createList([1,2,3,4])
    printList(reverseList_v2(list4))
    
    list2 = createList([1,2,3])
    printList(reverseList_v2(list2))
    


# Idea: First reverse the link and then make a recursive call
# Going left to right
# time - theta(N), space - theta(N) [N function calls in stack]
def reverseList_v2(curr, prev=None):
    # base case
    if curr == None:
        return prev
    
    # reverse curr and prev and then reverse(next, curr)
    next = curr.next 
    curr.next = prev 
    return reverseList_v2(next, curr)  
    

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