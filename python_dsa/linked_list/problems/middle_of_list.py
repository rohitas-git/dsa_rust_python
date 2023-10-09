class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    list1 = createList([])
    # print(middle(list1))
    efficientMiddle(list1)
    
    list2 = createList([1,2])
    # print(middle(list2))
    efficientMiddle(list2)

    
    list3 = createList([1,2,99,4])
    # print(middle(list3))
    efficientMiddle(list3)
    
    list4 = createList([1,2,11,4,5])
    # print(middle(list4))
    efficientMiddle(list4)

    

# time - theta(N) [one traversal], space - O(1) 
# two pointer approach
def efficientMiddle(head):
    if head == None:
        print(head)
        return 
    
    slow = head 
    fast = head 
    
    while fast != None and fast.next != None:
        slow = slow.next 
        fast = fast.next.next 
    
    print(slow.key)


# time - theta(N) [one traversal], space - theta(N)
# while traversing list, track values of node and find length of list
def middle(head):
    if head == None:
        return 
    count= 1
    val = []
    curr = head 
    while curr != None:
        val.append(curr.key)    
        curr = curr.next 
        count+=1
    if count == 2:
        return head.next.key 
    if count % 2 == 0:
        mid = count // 2
    else:
        mid = count //2 + 1
    return val[mid-1]


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


def insertBegin(list,target):
    toAdd = Node(target)
    toAdd.next = list 
    
    return toAdd

def printList(head):
    traveller = head 
    while traveller != None:    
        print(traveller.key)
        traveller = traveller.next 


main()