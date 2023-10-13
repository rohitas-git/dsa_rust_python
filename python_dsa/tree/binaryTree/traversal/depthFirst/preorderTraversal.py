# Preorder Traversal
# -> Root > Left > Right

# Linked representation of Binary tree
class Node:
    def __init__(self, data):
        self.left = None    # for left child
        self.right = None   # for right child
        self.data = data


def main():
    root = Node(10)
    root.left = Node(20)
    root.right = Node(30)
    root.right.left = Node(40)
    root.right.right = Node(50)
    
    # preorder(root)
    preorderSimpler(root)
    preorderFancier(root, "ROOT", 0)
   

# Essence of Preorder way of Depth first traversal
# Time - THETA(N)
# Space - THETA(H) [at most H+1 entries in fn call stack, where H = height of tree from root to leaf]
def preorderSimpler(root):
    if root:
        print(root.data)
        preorder(root.left)
        preorder(root.right)  



# Time - THETA(N) [N* theta(1)] (N - total no. of nodes in tree)
# Space - THETA(H) [at most H entries in fn call stack]
def preorder(root):
    if root == None:
        return 
    
    # base case
    if root.left == None and root.right == None:
        print(root.data)
        return 
    
    # traverse root node
    print(root.data)
        
    # traverse left subtree
    preorder(root.left)
    
    # traverse right subtree
    preorder(root.right)    
    
    

def preorderFancier(root, area, depth):
    if root == None:
        return 
    
    # base case
    if root.left == None and root.right == None:
        print(depth, area, end=" ")
        print(root.data)
        return 
    
    # traverse root node
    rt = area 
    print(depth, rt, root.data)
        
    # traverse left subtree
    depth+=1
    l= area + "-LEFT"
    preorderFancier(root.left, l, depth)
    
    # traverse right subtree
    # if r.split("-")[-1] =="LEFT":
    r = area + "-RIGHT"
    preorderFancier(root.right, r, depth) 
    
    
#Function to return a list containing the preorder traversal of the tree.
def preorder(root):
    if root:
        result=[]
        result.append(root.data)
        la = preorder(root.left)
        ra = preorder(root.right)
        if la:
            for i in la:
                result.append(i)
        if ra:
            for i in ra:
                result.append(i)
        return result
    

main()