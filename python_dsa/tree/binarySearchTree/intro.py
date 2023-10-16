
# Binary Search Tree

# For every Node,
    # - keys to Left   < Node
    # - keys to Right  > Node

# All keys are typically considered as distinct 

# In General:
# Search,Insert,Delete - O(h)
# Finding closest (ceiling/floor) - O(h)

# To optimize the height of tree => Balanced BST


class Node:
    def __init__(self, data):
        self.left = None    # left.data < self.data
        self.right = None   # right.data > self.data
        self.data = data
