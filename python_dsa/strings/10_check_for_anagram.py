
# anagram if s2 is a permutation of s1
# i.e both strings have same freq count of chars in their string

# Naive soln: time O(nlogn)
def areAnagram_naive(s1,s2):
    if len(s1) != len(s2):
        return False
    s1 = sorted(s1)
    s2 = sorted(s2)
    return (s1 == s2)


# time - theta(len(s1))
def areAnagram(s1,s2):
    if len(s1) != len(s2):
        return False
    
    count = [0]*256
    for i in range(len(s1)):
        count[ord(s1[i])] +=1
        count[ord(s2[i])] -=1
        
    for x in count:
        if x != 0:
            return False
    return True