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
    head.next.next.next = Node(50)    
    
    target = 30
    print(search(head, target))
    assert search(head, target) == 3
    
    print(search(head, 40))
    assert search(head, 40) == -1
    
    
# Worst case - theta(N)
# Best case - theta(1)
# time - O(N), space O(1)
def search(list,target):
    current = list
    pos = 1
    print("-----LINKED LIST------")
    while current != None:
        if current.key == target:
            return  pos
        else:
            current = current.next 
            pos+=1
    
    return -1


main()