# ------------------------------ Open Addressing ----------------------------- #

# Idea to use single array only
# which leads to the requirement that:
#  - no. of slots > no. of keys to be inserted

# Adv: 
#  - Cache Friendly

# Multiple ways to impl it due to diff ways to handle collisions:
#  - Linear Probing, Quadratic Probing, Double Hashing

# Example here uses linear probing

# Linear Probing:
# When there is a collision, linearly search for next empty slot

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
# Quadratic Probing
# hash(key,i) = (h(key) + i*i) mod m; => Results in Secondary Clusters
# 
# Better than Linear Probing
# But in quadractic probing, it's not guaranteed that empty slots can be found even if they are present
# Guarantee that it works if
# - load factor < 0.5 
# - Table size(m) is a prime number

# Solution 2:
# Double Hashing:
# hash(key,i) = (h1(key) + i* h2(key)) mod m