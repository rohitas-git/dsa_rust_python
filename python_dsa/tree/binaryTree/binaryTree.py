
# Binary Tree: Every node has atmost two children
# Representation: -linked -array
# Degree of a tree: the maximum degree of a node in the tree
    # For Binary tree - 2 

# Why only binary tree popular and not ternary tree?
    # Because binary tree are helpful in binary searching. 
    # Binary search is the best way to divide your range into two halves
    # Dividing into 3 parts is not going to be more efficient than binary search 

# Linked representation of Binary Tree
class Node:
    def __init__(self, data):
        self.left = None    # for left child
        self.right = None   # for right child
        self.data = data


# Instance of BTree
root = Node(10)
root.left = Node(20)
root.right = Node(30)
root.left.left = Node(40)
