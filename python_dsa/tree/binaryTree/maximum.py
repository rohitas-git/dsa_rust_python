
# Size of Binary Tree 
# -> number of nodes in the tree

# To calculate size, follow any of the traversals

import math

class Node:
    def __init__(self, data):
        self.left = None    # for left child
        self.right = None   # for right child
        self.data = data


# time - theta(n), space - theta(h)
def getMax(root):
    if root == None:
        return -math.inf
    else:
        lm = getMax(root.left)
        rm = getMax(root.right)
        return max(lm,rm,root.data)
    
    

# Instance of BTree
root = Node(10)
root.left = Node(20)
root.right = Node(90)
root.left.left = Node(40)

print(getMax(root))