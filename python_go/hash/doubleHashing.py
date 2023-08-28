# Double Hashing:

# To find position of key in an array of size m(prime number) at ith probing ->
# Hash(key,i) = (h1(key) + i*h2(key)) % m 

# - If h2(key) is relatively prime to m, then it always find a free slot if there is one
# - Distribute keys more uniformly than linear probing and quadratic probing
# - No clustering

# Ex

# m = 7
# h1(key) = (key % 7)
# h2(key) = 6 - (key % 6)       [so that it's not zero for any key]

# Why need to find h2(key) and m relatively prime?
# It allows (h2(key) % m) gives the offset/shift when probing
# and it range [1 to m-1] 
# It results in a generalized version of linear probing

# Performance Analysis of Search 
# alpha = n/m (for open addressing, alpha <= 1)
# assumption: Every probe sequence looks at a random location

# Successful

# Unsuccessful search:
# (1-alpha) = fraction of table that is empty
# Expected number of probes required = 1 / (1- alpha) 
# alpha = 0.8 => avg of 5 probes req
# alpha = 0.9 => avg of 10 probes req


# Like Chaining, the performance of hashing can be evaluated under the assumption that each key is equally likely to be hashed to any slot of the table (simple uniform hashing) 

# m = Number of slots in the hash table

# n = Number of keys to be inserted in the hash table

#  Load factor α = n/m  ( < 1 )

# Expected time to search/insert/delete < 1/(1 – α) 

# So Search, Insert and Delete take (1/(1 – α)) time