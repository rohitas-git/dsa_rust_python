# Inorder Traversal
# -> Left > Root > Right

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

    # inorder(root)
    inorderSimpler(root)
    inorderFancier(root, "ROOT", 0)


# Essence of Inorder way of Depth first traversal
# Time - THETA(N)
# Space - THETA(H) [at most H+1 entries in fn call stack, where H = no.of nodes from root to a leaf]
def inorderSimpler(root):
    if root:
        inorder(root.left)
        print(root.data)
        inorder(root.right)


# Time - THETA(N) [N* theta(1)] (N - total no. of nodes in tree)
# Space - THETA(H) [at most H entries in fn call stack]
def inorder(root):
    if root == None:
        return

    # base case
    if root.left == None and root.right == None:
        print(root.data)
        return

    # traverse left subtree
    inorder(root.left)

    # traverse root node
    print(root.data)

    # traverse right subtree
    inorder(root.right)


def inorderFancier(root, area, depth):
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

    inorderFancier(root.left, l, depth)

    # traverse root node
    depth -= 1
    rt = area
    print(depth, rt, root.data)

    # traverse right subtree
    depth += 1
    r = area + "-RIGHT"
    inorderFancier(root.right, r, depth)


def inorder_v2(root):
    if root:
        result = []

        la = inorder_v2(root.left)
        if la:
            for i in la:
                result.append(i)

        result.append(root.data)

        ra = inorder_v2(root.right)
        if ra:
            for i in ra:
                result.append(i)

        return result


main()
