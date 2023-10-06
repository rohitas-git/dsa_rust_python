class Node:
    def __init__(self, K):
        # data stored in node
        self.key = K 
        # reference to connected node
        self.next = None 
        

def main():
    head = Node(10)
    head.next = Node(20)
    head.next.next = Node(30)

    printList(head)

# time - Theta(N), space - O(1)
def printList(head):
    traveller = head 
    while traveller != None:    
        print(traveller.key)
        traveller = traveller.next 
        

main()
