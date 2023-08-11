

def main():
    # print1ToN(5)
    # printNto1(5)
    # print(sumOfDigits(8915))
    # print(is_palindrome("abbcbba"))
    print(is_palindrome("abba"))
    


def print1ToN(n):
    if n<=0:
        return
    print1ToN(n-1)
    print(n)
    
    
def printNto1(n):
    if n<=0:
        return
    print(n)
    printNto1(n-1)
    

def sumOfDigits(n):
    if n < 0:
        n = -1*n
    if n < 10:
        return n
    return sumOfDigits(n//10) + n % 10
    
    
def is_palindrome(s):
    if len(s) <= 1:
        return True
    
    if s[0] == s[-1]:
        return is_palindrome(s[1:len(s)-1])
    
    return False
    
    
main()