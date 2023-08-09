# -------------------------------- Python List ------------------------------- #
# Python Lists are just like dynamically sized arrays
# The list is a sequence data type which is used to store the collection of data. 
# Tuples and String are other types of sequence data types. 

# Lists are the simplest containers that are an integral part of the Python language. 
# Lists need not be homogeneous always which makes it the most powerful tool in Python. 
# A single list may contain DataTypes like Integers, Strings, as well as Objects. 
# Lists are mutable, and hence, they can be altered even after their creation.

# Complexities for Creating Lists
# Time Complexity: O(1)
# Space Complexity: O(n)

# Complexities for Accessing Lists
# Time Complexity: O(1)
# Space Complexity: O(1)

# ------------------------- List insert and search Fn ------------------------ #

l = [ (x * 10) for x in range(1,5)]

print(l)
print(l[1])
print(l[-1])

l.append(30)
l.insert(1,15)

print(l)
print(15 in l)  # true
print(l.count(30))  # 2
print(l.index(30))  # 3
print(l.index(30,4,7)) # 5

print(len(l)) # 6
