from sys import argv

def main():
    n = int(argv[1])
    a = isPalindrome(n)
    print(a)
    
    
def isPalindrome(n:int)->bool:
    digits = []
    number = n
    
    while n > 0:
        digit = n % 10
        n = n // 10
        digits.append(digit)

    # print(digits)
    ll = len(digits)
    l =  ll // 2
    for i in range(l):
        # print(digits[i], digits[ll-1-i])
        if digits[i] != digits[ll-1-i]:
            return False
    return True


main()