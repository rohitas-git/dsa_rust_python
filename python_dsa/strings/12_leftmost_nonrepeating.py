
# Naive soln: O(n*n)
def leftmost_nonrepeating(str):
    for i in range( len(str) ):
        flag = False
        for j in range(i+1, len(str)):
            if str[i] == str[j]:
                flag = True
                break
        if flag == False:
            return i
    return -1


# consider all ascii characters
CHARS = 256

# Better soln: O(n) (two traversals)
# Approach: Use char as an index and do freq count of it
def leftmost_nonrepeating(str):
    count=[0]*CHARS
    
    for i in range(len(str)):
        count[ord(str[i])] +=1
    for i in range(len(str)):
        if count[ord(str[i])] == 1:
            return i 
    return -1


# Better soln: 
# theta(n + CHARS), theta(CHARS)
# O(n) (one traversals of N + one traversal of CHARS)
from sys import maxsize
def leftmost_nonrepeating(str):
    first_index = [-1]*CHARS
    for i in range(len(str)):
        if first_index[ord(str[i])] == -1:
            first_index[ord(str[i])] = i
        else:
            first_index[ord(str[i])] = -2
    
    res = maxsize
    for i in range(CHARS):
        if first_index[i] > 0:
            res = min(res,first_index[i])
    if res == maxsize:
        return -1
    else:
        return res
    
