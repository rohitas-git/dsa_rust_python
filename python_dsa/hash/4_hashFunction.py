
# * Hashing
# Hashing is a technique or process of mapping keys, and values into the hash table by using a hash function. 
# It is done for faster access to elements. 
# The efficiency of mapping depends on the efficiency of the hash function used.

# ! What is Hash function?
# The hash function receives the input key and 
# returns the index of an element in an array called a hash table. 
# The index is known as the hash index.

# ! Purpose of Hash function
# Take Large Universe of Keys
#  --> [Hash Func] -->
# Map them to small values in a limited range that can be used as index of array


# How to write Hash Functions?

# ! Properties req of Hash fn:
# - Should be determinstic (same large key is always mapped to same small value)
# - Should only generate values from 0 to M-1  if M is size of hash table
# - Should be Fast ( O(1) for Integers, O(len(string)) for strings )
# - Should uniformly distribute large keys into hash table slots (toughest part to achieve)

# ! Size of hashtable dependent on the number of input keys
# if from universe of phone numbers, we have to store 100 numbers 
# then size of hashtable could be 100, 200, 400  

# Example Hash Functions:

# ! 1. The mod method: 
# In this method for creating hash functions, we map a key into one of the slots of table by 
# taking the modulo of key  with table_size.
# h(largeKey) = large_key % m     

# ! (typically m is chosen as prime number close to hashtable size)
# WHY?
# m as prime number results in less common factos and better distribution of large keys
# bad value of 2^_ or 10^_ ;

# ! we choose prime numbers that not close to powers of 2 and 10
# WHY?
# m = 2^3 means we taking values of last 8 bits in binary repr of large key 
# m = 10^3 means we taking last 3 digits of large key as hashtable bucket/slot

# Disclaimer: NOT A GOOD HASH FN >> h(largeKey) = large_key % m    

# ! 2. For strings, weighted sum   
# str[] = "abcd"
# (str[0]*(x^0) + str[1]*(x^1) + str[2]*(x^2) + str[3]*(x^4)) % m , 
# where x can be a number like 33

# ! 3. Universal Hashing

# We have group of hash fn. 
# We randomly pick one and use that to hash all our data in the hashtable
# When inserting a new key, we again pick one hash fn randomly 

# WHY?
# The goal of universal hashing is to minimize the chance of collisions between distinct keys, 
# which can lead to degraded performance in hash table operations.
# The family of hash functions is designed to minimize the probability of collisions, regardless of the distribution of keys. 
# By randomly selecting a hash function from the family for each new key, the chance of collisions is further reduced.



# ! 4. The multiplication method: 
# In multiplication method, we multiply the key k by a constant real number c 
# in the range 0 < c < 1 and extract the fractional part of k * c.
# Then we multiply this value by table_size m and take the floor of the result. 
# It can be represented as
# h(k) = floor (m * (k * c mod 1))
                    #  or
# h(k) = floor (m * frac (k * c)) 
