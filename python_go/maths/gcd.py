from sys import argv


# GCD:-
# For number a and b,
# if a = e*d and b = f*d & d is the largest such integer 
# then it's the gcd of (a,b)

# Euclid theorem: gcd(a, b) = gcd(b, a%b)
def gcd_EuclidTheorem(a,b):
    if b == 0:
        return a
    return gcd_EuclidTheorem(b, a % b)

# Time Complexity: O(logɸmin(a,b)), where ɸ is 1.61.
# Space Complexity: O(1).
    
a = int(argv[1])    
b = int(argv[2])
print(gcd_EuclidTheorem(a,b))


# Brute force
# Intuition: Simply traverse from 1 to min(a,b) and check for every number.
if __name__ == '__main__':
    num1 = 4
    num2 = 8
    ans = 1
    for i in range(1, min(num1, num2) + 1):
        if num1 % i == 0 and num2 % i == 0:
            ans = i
    print("The GCD of the two numbers is", ans)