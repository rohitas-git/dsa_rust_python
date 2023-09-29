
def isPalindromeV2():
    s = input("Enter a string: ")
    if s == s[::-1]:
        print('Yes')
    else:
        print('No')

def isPalindrome():
    s = input("Enter a string: ")
    low = 0
    high = len(s)-1
    
    while low<high:
        if s[low] != s[high]:
            print('No')
            break
        low+=1
        high-=1
    else:
        print('Yes')

isPalindrome()