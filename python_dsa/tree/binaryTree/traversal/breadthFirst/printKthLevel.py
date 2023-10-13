
# Print nodes at K height 

# Linked representation of Binary tree
class Node:
    def __init__(self, data):
        self.left = None    # for left child
        self.right = None   # for right child
        self.data = data

    def addLeft(self, data):
        self.left= Node(data)
    
    def addRight(self, data):
        self.right= Node(data)
        

# time - O(N), space - O(h)
def printKth(root, k):
    if root == None:
        return 
    if k == 0:
        print(root.data, end=" ")
        return

    printKth(root.left, k-1)
    printKth(root.right, k-1)
    
root = Node(10)
root.left = Node(20)
root.right = Node(30)
root.right.left = Node(40)
root.right.right = Node(50)

printKth(root, 1)