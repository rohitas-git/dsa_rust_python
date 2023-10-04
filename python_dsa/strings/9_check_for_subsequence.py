
# subsequence 
# "ABC" - "" "A" "B" "C" "AB" "AC" "BC" "ABC" 
# [No. of subsequences = 2^n, where n=len(s)]

# time - O( len(s1) )
def inSubSeq(s1,s2):
    if (len(s2) > len(s1)):
        return False
    
    processed,matched = 0,0
    while (processed < len(s1) and matched < len(s2)):
        if s1[processed] == s2[matched] :
            matched+=1
        processed+=1
    if matched ==len(s2):
        return True
    else:
        return False
    
# time - O(n+m), space - O(min(m,n))
def inSubSeq(s1,s2,m,n):
    if n==0:
        return True
    if m==0:
        return False
    
    if s2[n-1] == s1[m-1]:
        return inSubSeq(s1,s2,m-1,n-1)
    else:
        return inSubSeq(s1,s2,m-1,n)
    
    

def check_subsequence():
    s1 = input("Enter string: ")
    s2 = input("Enter string to check for subsequence: ")
    
    for c in s2:
        found = s1.find(c)
        if (found != -1):
            s1 = s1[found:]
        else:
            print("No")
    return print('YES')
    
check_subsequence()