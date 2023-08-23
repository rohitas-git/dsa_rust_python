# Hashing
# Hashing is a technique or process of mapping keys, and values into the hash table by using a hash function. 
# It is done for faster access to elements. The efficiency of mapping depends on the efficiency of the hash function used.

# Take Large Universe of Keys
#  --> [Hash Func] -->
# Map them to small values in a limited range that can be used as index of array


# How to write Hash Functions?

# Properties req of Hash fn:
# - Should be determinstic (same large key is always mapped to same small value)
# - Should only generate values from 0 to M-1  if M is size of hash table
# - Should be Fast ( O(1) for Integers, O(len(string)) for strings )
# - Should uniformly distribute large keys into hash table slots

# Size of hashtable ~ number of input keys

# Example Hash Functions:

# ! 1. The mod method: 
# In this method for creating hash functions, we map a key into one of the slots of table by 
# taking the remainder of key divided by table_size.
# h(largeKey) = large_key % m     

# (typically m is chosen as prime number close to hashtable size)
# m as prime number results in less common factos and better distribution of large keys
# bad value of 2^_ or 10^_ ;
# ! we choose prime numbers that not close to powers of 2 and 10

# m = 2^3 means we taking values of last 8 bits in binary repr of large key 
# m = 10^3 means we taking last 3 digits of large key as hashtable bucket/slot

# ! 2. For strings, weighted sum   
# str[] = "abcd"
# (str[0]*(x^0) + str[1]*(x^1) + str[2]*(x^2) + str[3]*(x^4)) % m , 
# where x can be a number like 33

# ! 3. Universal Hashing

# Set of hash fn. We randomly pick one. 

# ! 4. The multiplication method: 
# In multiplication method, we multiply the key k by a constant real number c 
# in the range 0 < c < 1 and extract the fractional part of k * c.
# Then we multiply this value by table_size m and take the floor of the result. 
# It can be represented as
# h(k) = floor (m * (k * c mod 1))
                    #  or
# h(k) = floor (m * frac (k * c)) 
