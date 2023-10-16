# Search for matching data in Binary Search Tree

# Naive: Check data of every node with the target - O(n)
# Efficient: Do binary search over BST  - O(logN)
    # (able to do because of its structure)
    

# Recursive implementation
# time - O(h)
# auxiliary space - O(h)
def searchBST(root, target):
    if root:
        if root.data == target:
            return True
        if root.data < target:
            return searchBST(root.right,target)
        if root.data > target:
            return searchBST(root.left,target)
    else:
        return False   # base (when node is None)
    


# Iterative implementation
# time - O(h)
# auxiliary space - O(1)
def searchBST_iterative(root,target):
    while root != None:
        if root.data == target:
            return True
        elif root.data > target:
            root = root.left
        else:
            root = root.right  
    return False


def naive_search(root, target):
    if root:
        if root.data == target:
            return True 
        ls = naive_search(root.left, target)
        rs = naive_search(root.right,target)
        return ls or rs     
    else:
        return False 