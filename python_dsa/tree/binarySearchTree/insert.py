
# Insert in BST

# If key is present, then not inserting key as keys in BST should be unique
# If key is absent, then always make insertion at a leaf node


class Node:
    def __init__(self, data):
        self.left = None    # left.data < self.data
        self.right = None   # right.data > self.data
        self.data = data


def insertBST(root, key):
    # empty tree
    if root is None:
        return Node(key) 
    
    while root.next != None:
        if root.data == key:
            print("Already present")
            return 
        elif root.data > key:
            root = root.left
        else:
            root = root.right
    

# recursive implementation
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