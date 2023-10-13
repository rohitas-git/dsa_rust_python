
# Size of Binary Tree 
# -> number of nodes in the tree

# To calculate size, follow any of the traversals


class Node:
    def __init__(self, data):
        self.left = None    # for left child
        self.right = None   # for right child
        self.data = data


# time - theta(n), space - theta(h)
def treeSize(root):
    if root == None:
        return 0
    else:
        return 1 + treeSize(root.left) + treeSize(root.right) 
    
    

# Instance of BTree
root = Node(10)
root.left = Node(20)
root.right = Node(30)
root.left.left = Node(40)

print(treeSize(root))