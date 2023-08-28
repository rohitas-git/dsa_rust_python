# ---------------------------- Collision Handling: --------------------------- #
# Since a hash function gets us a small number for a big key, 
# there is possibility that two keys result in same value. 
# The situation where a newly inserted key maps to an already occupied slot 
# in hash table is called collision and must be handled using some collision handling technique.

# If we know the keys in advance, then we can Perfect Hashing (ensures that there zero collisions)
# If we do not know the keys in advance, then we can use one of the following:
# ! - Chaining
# ! - Open Addressing 

# Birthday Paradox 
# If 23 people in a room then there is 50% probability that two people in the room have same birthday 
# If 70 people, then 99.9 %

# If you don't know keys in advance then collisions are bound to happen
# Can be seen from Birthday Paradox

# --------------------------------- Chaining --------------------------------- #
# The idea behind separate chaining is to implement the array as a linked list called a chain. 
# Separate chaining is one of the most popular and commonly used techniques in order to handle collisions.

# ------------------------------ Open Addressing ----------------------------- #
# In Open Addressing, all elements are stored in the hash table itself.
# This approach is also known as closed hashing. This entire procedure is based upon probing.