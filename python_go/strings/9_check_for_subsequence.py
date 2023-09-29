
# subsequence 
# "ABC" - "" "A" "B" "C" "AB" "AC" "BC" "ABC" 
# [No. of subsequences = 2^n, where n=len(s)]




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