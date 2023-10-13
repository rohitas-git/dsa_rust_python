# Postorder Traversal
# ->  Left > Right > Root
# It's not tail recursion like inorder and preorder

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

    # postorder(root)
    postorderSimpler(root)
    postorderFancier(root, "ROOT", 0)


# Essence of Postorder way of Depth first traversal
# Time - THETA(N)
# Space - THETA(H) [at most H+1 entries in fn call stack, where H = height of tree from root to leaf]
def postorderSimpler(root):
    if root:
        postorder(root.left)
        postorder(root.right)
        print(root.data)


# Time - THETA(N) [N* theta(1)] (N - total no. of nodes in tree)
# Space - THETA(H) [at most H entries in fn call stack]
def postorder(root):
    if root == None:
        return

    # base case
    if root.left == None and root.right == None:
        print(root.data)
        return

    # traverse root node
    print(root.data)

    # traverse left subtree
    postorder(root.left)

    # traverse right subtree
    postorder(root.right)


def postorderFancier(root, area, depth):
    if root == None:
        return

    # base case
    if root.left == None and root.right == None:
        print(depth, area, end=" ")
        print(root.data)
        return

    # traverse left subtree
    depth += 1
    l = area + "-LEFT"
    postorderFancier(root.left, l, depth)

    # traverse right subtree
    # if r.split("-")[-1] =="LEFT":
    r = area + "-RIGHT"
    postorderFancier(root.right, r, depth)

    # traverse root node
    depth -= 1
    rt = area
    print(depth, rt, root.data)


def postorder_v2(root):
    if root:
        result=[]
        
        la = postorder(root.left)
        if la:
            for i in la:
                result.append(i)
                
        
        ra = postorder(root.right)
        if ra:
            for i in ra:
                result.append(i) 
                
        result.append(root.data)
        
        return result 
    
    
main()
