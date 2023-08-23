# Idea of Hashing is based on simple concept of direct address table

# --------------------------- Direct Address Table --------------------------- #

""" 
Direct Address Table is a data structure 
that has the capability of mapping records to their corresponding keys using arrays. 
In direct address tables, records are placed using their key values directly as indexes. 
They facilitate fast searching, insertion and deletion operations. 
"""


# Imagine a situation where you have 1000 keys with values from (0 to 999), 
# how would you implement following in O(1) time
# - search - insert - delete

# if keys are in [0+k : 1000 + k]
# for [0:1000] -> k =0
# for [1000:2000] -> k=1000

directAddressTable = [0]*(1000)

def search(num, k):
    return directAddressTable[num-k]

def insert(num,k):
    directAddressTable[num-k] = 1

def delete(num,k):
    directAddressTable[num,k] = 0

# Search, delete, insert - O(1) for direct address table
# Useful when keys are in small range

# The idea is to use keys as indexes in an array 
# instead of a = [10,125,215]
# say, if we knw that values in a are in range(0,250):
# we could make an array = [0]* 250 and have arr[10/125/215] = 1 to mark that these values are present

# So, what is the problem with it??

# Problem with Direct Address Table: It doesn't handle
# - Large key values (ex: storing phone numbers 10-digit number would req 10^10 keys )
# - floating point numbers can not be used as index in array
# - strings or address or combination of digits and characters 

# Limitations:
# 
# - Prior knowledge of maximum key value
# - Practically useful only if the maximum value is very less.
# - It causes wastage of memory space if there is a significant difference between total records and maximum value.

# Hashing can overcome these limitations of direct address tables. 

# The only difference from hashing here is, 
# we do not use a hash function to find the index. 
# We rather directly use values as indexes.