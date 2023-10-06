class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    head = None
    head = insertBegin(head, 10)
    head = insertBegin(head, 20)
    head = insertBegin(head, 30)

    printList(head)
    

# time - O(1), space O(1)
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