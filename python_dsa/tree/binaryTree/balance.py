# Given a binary tree, find if it is height balanced or not. 
# A tree is height balanced 
# if difference between heights of left and right subtrees is not more than one for all nodes of tree. 

class Node:
    def __init__(self, data):
        self.left = None    # for left child
        self.right = None   # for right child
        self.data = data


def height(root):
    if root == None:
        return 0
    else:
        lh = height(root.left)
        rh = height(root.right)
        return max(lh, rh) + 1


def isBalanced(root):
    if root == None:
            return True
    else:
        lh = height(root.left)
        rh = height(root.right)
        
        if abs(lh-rh) > 1:
            return False
            
        lb = isBalanced(root.left)
        rb = isBalanced(root.right)
        
        if lb and rb:
            return True
        else:
            return False


    
# Instance of BTree
root = Node(10)
root.left = Node(20)
root.right = Node(30)
root.left.left = Node(40)

