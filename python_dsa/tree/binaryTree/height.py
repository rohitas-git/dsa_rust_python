# Height of Binary Tree

# Two conventions to find it:
# 1. Maximum number of nodes from root to leaf path
# 2. Maximum number of edges from root to leaf path

# Note: Edges = Nodes-1

# Linked representation of Binary Tree
class Node:
    def __init__(self, data):
        self.left = None    # for left child
        self.right = None   # for right child
        self.data = data


# following 1st convention
# time - theta(N), space - theta(H)
def height(root):
    if root == None:
        return 0
    else:
        lh = height(root.left)
        rh = height(root.right)
        return max(lh, rh) + 1
  

# following 2nd convention
# time - theta(N), space - theta(H)
def height_c2(root):
    if root == None:
        return -1
    else:  
        lh = height(root.left)
        rh = height(root.right)
        return max(lh, rh) + 1
    
root = Node(10)
root.left = Node(20)
root.right = Node(30)
root.left.left = Node(40)

print(height(root))
