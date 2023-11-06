
# Find Floor of a data in BST
#  -> Closest smaller or equal to value  


# import sys
# sys.path.insert(0, '/home/user/dev/ossu/ggDSA/python_dsa/tree/binaryTree/traversal/depthFirst')

# from inorderTraversal import inorderFancier

class Node:
    def __init__(self, data):
        self.left = None    # left.data < self.data
        self.right = None   # right.data > self.data
        self.data = data


def main():
    root = None   
    for i in [50,20,40,70,60,80]:
        root = insertBST(root,i)
    

    



# recursive implementation
# time - O(h), aux space - O(h)
def getFloor(root,target):
    curr = root 
    parent = None
    while curr != None:
        parent = curr 
        if curr.data == target:
            return target 
        if curr.left != None:
            if curr.left.data < target 
                


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