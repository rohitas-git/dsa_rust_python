class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    head = None
    head = insertBegin(head, 101)
    head = insertBegin(head, 202)
    head = insertBegin(head, 303)
    head = insertBegin(head, 404)
    head = insertBegin(head, 505)
    printList(head)
        
    head = deleteFirstNode(head)
    printList(head)
    
    head = deleteLastNode(head)    
    printList(head)
    
    deleteGivenNode(getNode(head,2))
    printList(head)
    

# time - O(1), space O(1)
def deleteFirstNode(head):
    # 1. case: empty list
    # 2. case: only one node in list
    # 3. case: more than one node in list
    print("DELETE FIRST NODE")
    if head == None:
        return None
    else:
        return head.next 


# time - Theta(N), space - O(1)
def deleteLastNode(head):
    # 1. case: empty list
    # 2. case: only one node in list
    # 3. case: more than one node in list
    
    print("DELETE LAST NODE")
    # case 1
    if head == None:
        return None
    # case 2
    if head.next == None:
        return None 
    # case 3
    else:
        # move to second-last node
        curr = head 
        while curr.next.next != None: 
            curr = curr.next
        curr.next = None
        return head 


# not given head, only given address of node to be deleted 
# may assume that node to delete will not be last node 
def deleteGivenNode(toDelete):
    next_node = toDelete.next 
    
    print("DELETE GIVEN NODE")
    
    # copy next node to current node    
    toDelete.key = next_node.key 
    toDelete.next = next_node.next 
    

def insertBegin(list,target):
    toAdd = Node(target)
    toAdd.next = list 
    
    return toAdd


def getNode(head, pos):
    p = 1
    curr = head 
    while curr != None:
        if p == pos:
            return curr 
        curr = curr.next 
        p += 1 
    return None 

def printList(head):
    traveller = head 
    print("--------------")
    while traveller != None:    
        print(traveller.key)
        traveller = traveller.next 
        
    print("--------------")


main()