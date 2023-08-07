from sys import argv

def main():
    n = int(argv[1])
    a = isPalindrome(n)
    aa = isPalindrome2(n)
    aaa = isPalindrome3(n)
    print(a, aa, aaa)
    

# Brute Force
# Approach 1: Reversing the Entire Number
# Time Complexity: O(logN) for reversing N digits of input integer.
def isPalindrome(n:int)->bool:
    if n < 0 or (n != 0 and n % 10 == 0):
            return False
    
    reversed_num = 0
    temp = n

    while temp != 0:
        digit = temp % 10
        reversed_num = reversed_num * 10 + digit
        temp //= 10

    return reversed_num == n


# Optimal
# Approach 2: Reversing Half of the Number
# Time Complexity: O(logN) for reversing N/2 digits of input integer.
def isPalindrome2(x):
        if x < 0 or (x != 0 and x % 10 == 0):
            return False

        half = 0
        while x > half:
            half = (half * 10) + (x % 10)
            x = x // 10

        return x == half or x == half // 10


def isPalindrome3(x):
        x = str(x)
        return x == x[::-1]


def reverse(X):
    Y = 0
    while X > 0:
        # Extract the last digit
        digit = X % 10
        # Appending last digit
        Y = Y * 10 + digit
        # Shrinking X by discarding the last digit
        X = X // 10
    return Y



if __name__ == "__main__":
    X = 121
    dummy = X
    Y = reverse(X)
    if dummy == Y:
        print("Palindrome Number")
    else:
        print("Not Palindrome Number")
        
# Output: Palindrome Number
# Time Complexity: O(logN) for reversing N digits of input integer.
# Space Complexity: O(1)

main()