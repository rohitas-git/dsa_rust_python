# ------------------------------ Open Addressing ----------------------------- #

# Like separate chaining, open addressing is a method for handling collisions. 
# ! In Open Addressing, all elements are stored in the hash table itself. 
# So at any point, the size of the table must be greater than or equal to the total number of keys (Note that we can increase table size by copying old data if needed). 
# This approach is also known as closed hashing. This entire procedure is based upon probing.

# Idea to use single array only
# which leads to the requirement that:
#  - no. of slots >= no. of keys to be inserted
#  => load factor, alpha <= 1

# Adv: 
#  - Cache Friendly

# Types of Probing
# - Linear Probing, 
# - Quadratic Probing, 
# - Double Hashing

# Example here uses linear probing

# ! Linear Probing:
# The hash table is searched sequentially that starts from the original location of the hash. 
# If in case the location that we get is already occupied, then we check for the next location. 
# rehash(key) = (n+1)%table-size

# Ex:
# i = key % 7 , keys = 48, 42, 50, 55, 53
# [48] [50] [55] [] [53] [] [48]  (
    # 55 % 7 = 6 -> collides with 48 % 7, so we search next empty slot in circular manner)
    
# Search in linear probing hash table:
# - Compute the hash fn to get the index 
# - If value at index is equal to target, return true 
# - Else, we do linear search while skipping deleted slots
#       till we find target or do 1 traversal of table or reach empty slot 

# In deletion, we marked the slot as deleted

# ------------------------ Problem with Linear Probing ----------------------- #

# Clustering 
# leads to more linear searching in search, insert, delete operations
# thus, affects performance of table

# Linear Probing to find position of key at ith probing can written as: 
# hash(key,i) = (h(key) + i) mod 7 ; h(key) = key % 7
# => Results in Primary Clusters

# Clustering happens because probe sequence are fixed for every collision

# Solution 1: 
# ! Quadratic Probing
# Quadratic probing is a method with the help of which we can solve the problem of clustering
# This method is also known as the mid-square method. 
# In this method, we look for the i2‘th slot in the ith iteration. 
# We always start from the original hash location. 
# If only the location is occupied then we check the other slots.

# hash(key,i) = (h(key) + i*i) mod m; => Results in Secondary Clusters

# Better than Linear Probing
# But in quadractic probing, it's not guaranteed that empty slots can be found even if they are present
# Guarantee that it works if
# - load factor < 0.5  => (size of table) > 2*(number of keys)
# - Table size(m) is a prime number

# Solution 2:
# ! Double Hashing:
# The intervals that lie between probes are computed by another hash function. 
# Double hashing is a technique that reduces clustering in an optimized way. 
# In this technique, the increments for the probing sequence are computed by using another hash function. 
# We use another hash function hash2(x) and look for the i*hash2(x) slot in the ith rotation. 

# hash(key,i) = (h1(key) + i* h2(key)) mod m