# --------------------------------- Chaining --------------------------------- #

# Idea: We maintain an array of linked list
# So what happens is, when multiple elements are hashed into the same slot index, 
# then these elements are inserted into a singly-linked list which is known as a chain

# Hash Table - Array of Linked List Headers

# -------------------------------- Performance ------------------------------- #
# m - no. of slots in hashtable
# n - no. of keys to be inserted

# Load factor, alpha - n/m 
#   Lower the load factor, lower the chances of collision

# Expect chain length 
#   - Worst case = n (all keys in one slot)
# Assuming uniform distribution of keys in table,  

# Expect chain length = Load factor (alpha)
# Expect time to search = O(1 + alpha)
# Expect time to insert/delete = O(1 + alpha)

# --------------------- Data structures of storing chain --------------------- #

# - Linked List 
#   Disadvantages:
#   {search,insert,delete - O(l), where chainlength is l} 
#   {Not cache friendly} 
#   {uses extra space for storing next references}

# - Dynamic Sized Array
#   Disadvantage same as Linked List
#       - No order in storing keys result in O(l) for search,insert,delete

# - Self-balancing BST (AVL tree, Red Black Tree)
#   better in search,snsert,delete - O(log l)
#   but not cache friendly