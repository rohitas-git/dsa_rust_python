# ------------------------------ Working of List ----------------------------- #

# list - a container to store collection of items


# A list mainly uses array as an underlying data structure
# Every member of the list  is actually reference

# List is actually an array of references
# References are stored contiguously but actual items might not be contiguous locations.

# Advantages
# - Random Access (get ith item in constant time)
# - Cache friendly (locality of reference concept)

# Disadvantage
# - Insertion, Deletion are slow/costly
# - Search for unsorted list are slow (O(n))

# -------------------------- Dynamic Size working ? -------------------------- #

# - Preallocate extra space 
# - If it becomes full, do the following :
    # - allocate a new space of larger size (depending on implementation)
    # - copy old space to the new 
    # - free old space 
    
# --------------------------- Append Amortized Time -------------------------- #

# For appending at the end - theta(1)
# 
# avg time to append (n+1) items = { {theta(1) + theta(1) ...} n times + theta(n) }/ (n+1)

# Pop and append are theta(1) on Average