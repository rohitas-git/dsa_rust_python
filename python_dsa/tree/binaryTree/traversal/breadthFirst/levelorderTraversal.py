
# ! Level Order Traversal

# naive solution: time O(n+nh)= O(nh)
# h = height(root)
# for k in range(h):
#   printKth(root,k)

# Recursive solution will not work
# - because it will process one subtree before processing node on same level 

# Iterative solution:
# Idea: use queue data structure
# REPEAT:
# 1 Put an item
# 2 Take out last item
# 3 Put children of that item in queue

# ----------------------------------------------------------------

from collections import deque

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
        

# time Theta(n), space O(n) 
# [aux space is equal to width of binary tree => theta(w)]
def levelOrder(root):
    if root:
        queue = deque()
        queue.append(root)
        while len(queue) > 0:
            node = queue.popleft() 
            print(node.data, end=" ")
            putChildrenInQueue(queue,node) 
        return 
    


def putChildrenInQueue(queue,node):
    if node.left:
        queue.append(node.left)
    if node.right:
        queue.append(node.right)
    return 
    

root = Node(10)
root.left = Node(20)
root.right = Node(30)
root.right.left = Node(40)
root.right.right = Node(50)

levelOrder(root)