# consider all ascii characters
CHARS = 256

#only lowercase chars
LOWERCASE_CHARS=26

# Naive soln: O(n*n)
def leftmost_repeating(str):
    for i in range( len(str) ):
        for j in range(i+1, len(str)):
            if str[i] == str[j]:
                return i
    return -1


# Better soln: O(n)  (two traversals)
# Approach: Use char as an index and do freq count of it
def leftmost_repeating(str):
    count=[0]*CHARS
    
    for i in range(len(str)):
        count[ord(str[i])] +=1
    for i in range(len(str)):
        if count[ord(str[i])] > 1:
            return i 
    return -1


# Better soln 2: Time O(n) [one traversal], Aux space O(1)
from sys import maxsize
def leftmost_repeating(str):
    first_index = [-1]*CHARS
    res = maxsize
    for i in range(len(str)):
        if first_index[ord(str[i])] == -1:
            first_index[ord(str[i])] = i
        else:
            res = min(res, first_index[ord(str[i])])
    if res == maxsize:
        return -1
    else:
        return res
    
    
# Best soln : 
# time O(n) [one traversal and fewer comparisons], 
# aux space O(1)
def leftmost_repeating(str):
    visited = [False]*CHARS
    res=-1
    for i in range(len(str)-1,-1,-1):
        if visited[ord(str[i])]:
            res = i
        else:
            visited[ord(str[i])] = True
    return res

a = leftmost_repeating("abccbda")
print(a)