
# Function to check whether all nodes of a tree have the value
# equal to the sum of their child nodes.

def isSumProperty(self, root):
    # code here
    if root:
        rt, l, r = 0, 0, 0

        rt = root.data

        if not root.left and not root.right:
            return 1
        if root.left:
            l = root.left.data
        if root.right:
            r = root.right.data

        if rt != (l+r):
            return 0

        ls, rs = True, True
        if root.left:
            ls = self.isSumProperty(root.left)
        if root.right:
            rs = self.isSumProperty(root.right)

        if ls and rs:
            return 1
        else:
            return 0
