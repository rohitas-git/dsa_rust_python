
# Insert in BST

# If key is present, then not inserting key as keys in BST should be unique
# If key is absent, then always make insertion at a leaf node

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
    
    # root = insert(root, 40)
    # insert(root, 20)
    # insert(root, 30)
    # insert(root, 10)
    # insert(root, 60)
    # insert(root, 50)
    # insert(root, 70)
    
    for i in [40,20,30,10,60,50,70]:
        root = insert(root,i)
    
    inorderFancier(root,"ORIGIN",0)


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
    

# recursive implementation
# time - O(h), aux space - O(h)
def insert(root,key):
    if root is None:
        return Node(key)
    elif root.data == key:
        return root
    elif root.data > key:
        root.left = insert(root.left, key)
    else:
        root.right = insert(root.right,key)
    return root 


main()