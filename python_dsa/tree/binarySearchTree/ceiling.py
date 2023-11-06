
# Find Ceiling in BST

# ceiling of value - closest greater value 
# Find ceiling in BST with value range in [a,b]

# 1 target lies in range
# 2 target larger than largest in range
# 3 range is empty set
# 4 target is equal to a element in range

# Soln:
# Naive: Traverse the BST and keep track of value greater than target 

class Node:
    def __init__(self, data):
        self.left = None    # left.data < self.data
        self.right = None   # right.data > self.data
        self.data = data


def main():
    root = None 
    for i in [40,20,30,10,60,55,50,70]:
        root = insertBST(root,i)
        
    ceiling(root, 35)
    ceiling(root, 58)
    ceiling(root, 8)
    ceiling(root, 60)
    

# Efficient: rely on BST search
# time - O(h) , aux space - O(1)
def ceiling(root, target):
    ceiling = None
    
    while root != None:
        if root.data == target:
            ceiling = root
            break
        elif root.data > target:
            ceiling = root
            root = root.left
        else:
            root = root.right
            
    if ceiling:
        print(ceiling.data)
    else:
        print("None")
    return ceiling 


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