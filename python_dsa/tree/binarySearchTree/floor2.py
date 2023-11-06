
# Find Floor in BST

# Floor of value - closest smaller value 
# Find floor in BST with value range in [a,b]

# 1 target lies in range
# 2 target smaller than lowest in range
# 3 range is empty set
# 4 target is equal to a element in range

# Soln:
# Naive: Traverse the BST and keep track of value smaller than target 

class Node:
    def __init__(self, data):
        self.left = None    # left.data < self.data
        self.right = None   # right.data > self.data
        self.data = data


def main():
    root = None 
    for i in [40,20,30,10,60,55,50,70]:
        root = insertBST(root,i)
        
    floor(root, 35)
    floor(root, 58)
    floor(root, 8)
    floor(root, 60)
    

# Efficient: rely on BST search
# time - O(h) , aux space - O(1)
def floor(root, target):
    floor = None
    
    while root != None:
        if root.data == target:
            floor = root
            break
        elif root.data > target:
            root = root.left
        else:
            floor = root
            root = root.right
    if floor:
        print(floor.data)
    else:
        print("None")
    return floor 


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