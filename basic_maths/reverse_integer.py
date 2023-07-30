from sys import argv

def main():
    n = int(argv[1])
    print(reverse(n))



# Time complexity: 
# Space complexity:
def reverse(x: int) -> int:
    res = 0

    sign = -1 if x < 0 else 1

    x *= sign

    while x>0:
        res = res * 10 + x % 10
        x //=10

    if sign * res < -2**31 or sign * res > 2**31 - 1:
        return 0
    
    return res * sign

main()