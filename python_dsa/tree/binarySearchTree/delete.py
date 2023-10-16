
# Delete in BST

# When deleting a node, need to ensure that BST is still BST
# Easily maintained when deleting a leaf node
# More complex if it's a internal node
    # If that node only has 1 child, place the only child in its place
    # If node has 2 child, then more than one way to proceed
        # Deleted node has to be replaced by closest value
            # - closest lower value
            # - closest upper value 

# closest greater value == inorder successor (next node that comes next inorder traversal)

# * Note: Inorder traversal of BST is always sorted

import sys
sys.path.insert(0, '/home/user/dev/ossu/ggDSA/python_dsa/tree/binaryTree/traversal/depthFirst')

from inorderTraversal import inorderFancier

class Node:
    def __init__(self, data):
        self.left = None    # left.data < self.data
        self.right = None   # right.data > self.data
        self.data = data


def main():
    root = None 
    
    # root = insert(root, 50)
    # insert(root, 20)
    # insert(root, 40)
    # insert(root, 10)
    # insert(root, 60)
    # insert(root, 50)
    # insert(root, 70)
    
    for i in [50,20,40,70,60,80]:
        root = insertBST(root,i)
    
    inorderFancier(root,"ORIGIN",0)
    
    print('\n -------------------------')
    delete(root, 20)
    inorderFancier(root,"ORIGIN",0)
    
    print('\n -------------------------')
    delete(root, 80)
    inorderFancier(root,"ORIGIN",0)
    
    print('\n -------------------------')
    delete(root, 50)
    inorderFancier(root,"ORIGIN",0)
    



# recursive implementation
# time - O(h), aux space - O(h)
def delete(root,target):
    # Traverse to the req. node
    # if key not present, do nothing; else
    # case 1: 0 child
    # case 2: 1 child
    # case 3: 2 child
    
    # traverse and acting based on decision
    if root == None:
        return 
    if root.data > target:
        root.left = delete(root.left, target)
    elif root.data < target:
        root.right = delete(root.right, target)
    else:
        # case 1 & 2         (in case 1: root.right== None)
        if root.left == None:
            return root.right
        # case 2
        elif root.right == None:
            return root.left 
        # case 3
        else:
            succ = getSuccessor(root.right,target)
            root.data = succ
            root.right = delete(root.right,succ)
    return root 
               
         
# successor is always a leaf node
def getSuccessor(curr,target):
    while curr.left != None:
        curr = curr.left 
    return curr.data




# iterative implementation
# time - O(h), aux space - O(1)
def insertBST(root, key):    
    parent = None
    curr = root 
    
    while curr != None:
        parent = curr 
        if curr.data == key:
            print("Already present")
            return root
        elif curr.data > key:
            curr = curr.left
        else:
            curr = curr.right
    
    if parent == None: 
        return Node(key)
    
    if parent.data > key:
        parent.left = Node(key)
    
    if parent.data < key:
        parent.right = Node(key) 
    
    return root
    



main()