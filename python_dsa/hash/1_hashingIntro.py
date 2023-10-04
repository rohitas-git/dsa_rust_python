# Hashing

# ! Hashing refers to
# the process of generating a fixed-size output from an input of variable size using the mathematical formulas known as hash functions. 
# This technique determines an index or location for the storage of an item in a data structure.

# There are majorly three components of hashing:
# * KEYS
# A Key can be anything string or integer which is fed as input in the hash function 
# the technique that determines an index or location for storage of an item in a data structure. 

# * HASH FUNCTION
# The hash function receives the input key 
# and returns the index of an element in an array called a hash table. 
# The index is known as the hash index.

# * HASH TABLE
# Hash table is a data structure that maps keys to values 
# using a special function called a hash function. 
# Hash stores the data in an associative manner in an array 
# where each data value has its own unique index.


# Mainly used to implement dictionary
# Hashing technqiue is used to create HashTables
# Also used to impl sets

# Search, Insert, Delete operations in => O(1) on average

# Unsorted Array:
# - Delete, Insert - O(1)
# - Search - O(N)

# Sorted Array
# - Search - O(logN)
# - Insert, Delete - O(N)

# Binary Search Tree (BST) like AVL and Red-Black tree
# - O(logN) for search, insert, delete

# Hash beats everyone in Search, Insert, Delete operations

# However, it doesn't provide efficient means for some operations 
# that are present in other data structures 
# Not useful for :
# - Finding closest value   [AVL or Red-black tree]
# - Sorted data     [AVL or Red-black tree]
# - Prefix searching    [Trie]

# ! Hashing is an excellent data structure when requirements are only Search, Insert, Delete 
# and when not interested in closest values and sorted order  
